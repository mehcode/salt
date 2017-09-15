mod route;
mod pattern;
mod parameters;

pub use self::route::Route;
pub use self::pattern::Pattern;
pub use self::parameters::Parameters;

use std::collections::HashMap;

use hyper::{self, Method, StatusCode};
use regex::RegexSet;
use futures::future;

use handler::Handler;
use context::Context;
use response::Response;
use ext::BoxFuture;

// From: https://github.com/crumblingstatue/try_opt/blob/master/src/lib.rs#L30
macro_rules! try_opt {
    ($e:expr) =>(
        match $e {
            Some(v) => v,
            None => return None,
        }
    )
}

#[derive(Default, Debug)]
pub struct Router {
    routes: HashMap<Method, Vec<Route>>,
    route_patterns: HashMap<Method, RegexSet>,
}

impl Router {
    /// Construct a new, empty `Router`.
    ///
    /// ```rust
    /// # use shio::router::Router;
    /// let router = Router::new();
    /// ```
    ///
    /// Equivalent to `Router::default()`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a new route to a `Router`, matching both a [`Method`] and a [`Pattern`]. Any value
    /// that implements `Into<Route>` may be provided to `route`.
    ///
    /// For example, to match a `Get` request to `/users`:
    ///
    /// ```rust
    /// # use shio::{Method, Response, StatusCode};
    /// # use shio::router::Router;
    /// # let mut router = Router::new();
    /// router.route((Method::Get, "/users", |_| {
    ///     // [...]
    /// # Response::with(StatusCode::NoContent)
    /// }));
    /// ```
    ///
    /// [`Method`]: https://docs.rs/hyper/0.11/hyper/enum.Method.html
    /// [`Pattern`]: struct.Pattern.html
    pub fn add<R: Into<Route>>(&mut self, route: R) {
        let route: Route = route.into();
        let method = route.method().clone();

        self.routes
            .entry(method.clone())
            .or_insert_with(Vec::new)
            .push(route);

        let routes = &self.routes[&method];

        // FIXME: Think of some way to not re-compile method maps so often during server boot
        // NOTE: The .unwrap cannot fail as we are using route patterns that are pre-verified
        self.route_patterns.insert(
            method.clone(),
            RegexSet::new(routes.iter().map(|r| r.pattern().as_str())).unwrap(),
        );
    }

    #[deprecated(since = "0.0.7", note = "use `Router::add` instead")]
    pub fn route<R: Into<Route>>(&mut self, route: R) {
        self.add(route);
    }

    pub fn find(&self, method: &Method, uri: &str) -> Option<&Route> {
        let routes = try_opt!(self.routes.get(method));
        let route_patterns = try_opt!(self.route_patterns.get(method));

        let route_index = try_opt!(route_patterns.matches(uri).into_iter().next());

        Some(&routes[route_index])
    }
}

impl Handler for Router {
    type Result = BoxFuture<Response, hyper::Error>;

    #[inline]
    fn call(&self, mut ctx: Context) -> Self::Result {
        // let route = self.find(ctx.method(), ctx.path());
        if let Some(route) = self.find(ctx.method(), ctx.path()) {
            // Re-parse the path to pull out captures
            if let Some(parameters) = route.pattern().parameters(ctx.path()) {
                // Add the parameters to the request context
                ctx.put::<Parameters>(parameters);
            } else {
                // NOTE: This shouldn't be possible to fail as we already matched against the
                //       path once.
            }

            route.call(ctx)
        } else {
            // Return 404 if we failed to find a matching route
            Box::new(future::ok(Response::with(StatusCode::NotFound)))
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio_core::reactor::Core;

    use super::{Parameters, Router};
    use {Context, Handler, Response, StatusCode};
    use Method::*;

    // Empty handler to use for route tests
    fn empty_handler(_: Context) -> Response {
        Response::with(StatusCode::NoContent)
    }

    /// Test for _some_ match for static in GET
    #[test]
    fn test_static_get() {
        let mut router = Router::new();
        router.add((Get, "/hello", empty_handler));

        assert!(router.find(&Get, "/hello").is_some());
        assert!(router.find(&Get, "/aa").is_none());
        assert!(router.find(&Get, "/hello/asfa").is_none());
    }

    /// Test for _some_ match for static in PUT, POST, DELETE
    #[test]
    fn test_static_put_post_del() {
        let mut router = Router::new();
        router.add((Put, "/hello", empty_handler));
        router.add((Post, "/hello", empty_handler));
        router.add((Delete, "/hello", empty_handler));

        assert!(router.find(&Get, "/hello").is_none());
        assert!(router.find(&Put, "/hello").is_some());
        assert!(router.find(&Post, "/hello").is_some());
        assert!(router.find(&Delete, "/hello").is_some());
    }

    /// Test for the correct match for static
    #[test]
    fn test_static_find() {
        // Correct match
        let mut router = Router::new();
        router.add((Get, "/aa", empty_handler));
        router.add((Get, "/hello", empty_handler));

        // FIXME: This section currently matches against regex
        //        This is an implementation detail; store the source strings and we'll
        //        match against that
        assert_eq!(
            router.find(&Get, "/hello").unwrap().pattern().as_str(),
            "^/hello$"
        );
        assert_eq!(
            router.find(&Get, "/aa").unwrap().pattern().as_str(),
            "^/aa$"
        );
    }

    /// Test for some match for segment parameter
    #[test]
    fn test_param_get() {
        let mut router = Router::new();
        router.add((Get, "/user/{id}", empty_handler));

        assert!(router.find(&Get, "/user/asfa").is_some());
        assert!(router.find(&Get, "/user/profile").is_some());
        assert!(router.find(&Get, "/user/3289").is_some());
        assert!(router.find(&Get, "/user").is_none());
    }

    /// Test for segment parameter value
    #[test]
    fn test_param_get_value() {
        let mut router = Router::new();
        router.add((Get, "/user/{id}", |context: Context| {
            // FIXME: We should have an assert that we got here
            assert_eq!(&context.get::<Parameters>()["id"], "3289");

            Response::with(StatusCode::NoContent)
        }));

        let mut core = Core::new().unwrap();
        let context = Context::builder(core.handle())
            .uri("/user/3289")
            .finalize().unwrap();

        let work = router.call(context);

        core.run(work).unwrap();
    }

    /// Test for some match for a custom parameter
    #[test]
    fn test_param_custom_get() {
        let mut router = Router::new();
        router.add((Get, "/static/{file: .+}", empty_handler));

        assert!(router.find(&Get, "/static").is_none());
        assert!(router.find(&Get, "/static/").is_none());
        assert!(router.find(&Get, "/static/blah").is_some());
        assert!(router.find(&Get, "/static/rahrahrah").is_some());
    }

    /// Test for segment parameter value
    #[test]
    fn test_param_get_custom() {
        let mut router = Router::new();
        router.add((Get, "/static/{filename: .*}", |context: Context| {
            // FIXME: We should have an assert that we got here
            assert_eq!(
                &context.get::<Parameters>()["filename"],
                "path/to/file/is/here"
            );

            Response::with(StatusCode::NoContent)
        }));

        let mut core = Core::new().unwrap();
        let context = Context::builder(core.handle())
            .uri("/static/path/to/file/is/here")
            .finalize().unwrap();

        let work = router.call(context);

        core.run(work).unwrap();
    }
}
