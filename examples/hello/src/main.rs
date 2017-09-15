extern crate shio;

use shio::prelude::*;

// Simple requests should be simple, even in the face of asynchronous design.
fn hello_world(_: Context) -> Response {
    Response::with("Hello World!\n")

    // This would be equivalent to returning `Response` and:

    /*
    const PHRASE: &str = "Hello World\n";

    Response::build()
        .header(header::ContentLength(PHRASE.len() as u64))
        .body(PHRASE)
    */

    // The default status code is `Status::Ok` (200).
}

fn hello(ctx: Context) -> Response {
    // Handlers have access to a type map which is a HashMap where the keys are types.
    //
    // With the default router, an instance of `Parameters` is present in this type map
    // that provides access to our route parameters.
    Response::with(format!("Hello, {}!", &ctx.get::<Parameters>()["name"]))
}

fn main() {
    // Construct a _default_ `Shio` service, mount the `index` handler, and
    // run indefinitely on port `7878` (by default, binds to both `0.0.0.0` and `::0`).
    Shio::default()
        .route((Method::GET, "/", hello_world))
        .route((Method::GET, "/{name}", hello))
        .run(":7878")
        .unwrap();

    // Shio services have an entry `Handler` that must be defined.
    // `Shio::default` constructs a `Shio` service with `shio::router::Router`
    // as its entry `Handler`.

    // This would be equivalent:

    /*

    let mut router = shio::router::Router::new();

    router.route((Method::GET, "/", hello_world));
    router.route((Method::GET, "/{name}", hello));

    let mut service = Shio::new(router);

    service.run(":7878").unwrap();

    */
}
