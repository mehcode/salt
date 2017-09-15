use std::fmt;

use hyper;
use futures::{future, Future, IntoFuture};

use response::Response;
use http::StatusCode;
use http::header::ContentLength;
use ext::{BoxFuture, FutureExt};
use handler::default_catch;

pub trait Responder
where
    <Self::Result as IntoFuture>::Error: fmt::Debug + Send + 'static,
{
    type Result: IntoFuture<Item = Response>;

    fn to_response(self) -> Self::Result;
}

impl Responder for () {
    type Result = Response;

    #[inline]
    fn to_response(self) -> Self::Result {
        Response::build().status(StatusCode::NoContent).into()
    }
}

impl<'a> Responder for &'a str {
    type Result = Response;

    #[inline]
    fn to_response(self) -> Self::Result {
        self.to_owned().to_response()
    }
}

impl Responder for String {
    type Result = Response;

    #[inline]
    fn to_response(self) -> Self::Result {
        Response::build()
            .header(ContentLength(self.len() as u64))
            .body(self)
    }
}

impl Responder for StatusCode {
    type Result = Response;

    #[inline]
    fn to_response(self) -> Self::Result {
        Response::build().status(self).into()
    }
}

impl<R: Responder> Responder for (StatusCode, R)
where
    <<R as Responder>::Result as IntoFuture>::Error: fmt::Debug + Send + 'static,
    <<R as Responder>::Result as IntoFuture>::Future: 'static,
{
    type Result = BoxFuture<<R::Result as IntoFuture>::Item, <R::Result as IntoFuture>::Error>;

    #[inline]
    fn to_response(self) -> Self::Result {
        let (status, responder) = self;
        responder
            .to_response()
            .into_future()
            .map(move |mut response| {
                response.set_status(status);
                response
            })
            .into_box()
    }
}

impl Responder for Response {
    type Result = Self;

    #[inline]
    fn to_response(self) -> Self::Result {
        self
    }
}

impl<E, R> Responder for Box<Future<Item = R, Error = E>>
where
    E: fmt::Debug + Send + 'static,
    R: Responder + 'static,
    <<R as Responder>::Result as IntoFuture>::Error: fmt::Debug + Send + 'static,
    <<R as Responder>::Result as IntoFuture>::Future: 'static,
{
    type Result = Box<Future<Item = Response, Error = hyper::Error>>;

    #[inline]
    fn to_response(self) -> Self::Result {
        self.then(|result| match result {
            Ok(responder) => responder
                .to_response()
                .into_future()
                .or_else(default_catch)
                .into_box(),
            Err(error) => future::ok(default_catch(error)).into_box(),
        }).into_box()
    }
}

impl<E, R> Responder for Result<R, E>
where
    E: fmt::Debug + Send + 'static,
    R: Responder + 'static,
    <<R as Responder>::Result as IntoFuture>::Error: fmt::Debug + Send + 'static,
    <<R as Responder>::Result as IntoFuture>::Future: 'static,
{
    type Result = BoxFuture<<R::Result as IntoFuture>::Item, hyper::Error>;

    #[inline]
    fn to_response(self) -> Self::Result {
        match self {
            Ok(responder) => responder
                .to_response()
                .into_future()
                .or_else(default_catch)
                .into_box(),
            Err(error) => future::ok(default_catch(error)).into_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use tokio_core::reactor::Core;
    use futures::{Future, IntoFuture, Stream};

    use super::{Responder, Response, StatusCode};

    fn to_response<R: Responder>(r: R) -> Response
    where
        <R::Result as IntoFuture>::Error: fmt::Debug + Send + 'static,
    {
        Core::new()
            .unwrap()
            .run(r.to_response().into_future())
            .unwrap()
    }

    fn assert_body(res: Response, expected: &str) {
        let mut core = Core::new().unwrap();

        let work = res.body().concat2().map(|body| {
            let body = String::from_utf8_lossy(&body);

            assert_eq!(expected, body);
        });

        core.run(work).unwrap();
    }

    #[test]
    fn str_to_response() {
        let res = to_response("Hello\n");

        assert_eq!(res.status(), StatusCode::Ok);
        assert_body(res, "Hello\n");
    }

    #[test]
    fn string_to_response() {
        let res = to_response(String::from("Hello\n"));

        assert_eq!(res.status(), StatusCode::Ok);
        assert_body(res, "Hello\n");
    }

    #[test]
    fn pair_to_response() {
        let res = to_response((StatusCode::Accepted, "Hello\n"));

        assert_eq!(res.status(), StatusCode::Accepted);
        assert_body(res, "Hello\n");
    }

    #[test]
    fn status_to_response() {
        let res = to_response(StatusCode::NoContent);

        assert_eq!(res.status(), StatusCode::NoContent);
        assert_body(res, "");
    }
}
