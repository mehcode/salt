//! Proxy google and stream the response back to the client.
//! This could be expanded into a simple http-proxy.

// TODO: Expand this a bit.
//  - Add some command line args: `proxy -p 7878 http://www.google.com`
//  - Add log and log when a request is proxied
//  - Use `Shio::new( .. )` as the router is just getting in the way here
//  - Proxy full request path

extern crate hyper;
extern crate shio;
#[macro_use] extern crate log;
extern crate simple_logger;

use hyper::Client;
use shio::prelude::*;

fn proxy(ctx: Context) -> BoxFuture<Response, hyper::Error> {
    // Additional work can be scheduled on the thread-local event loop,
    // as each handler receives a reference to it
    info!("Initialising new proxy request.");
    Client::new(ctx.handle())
        .get("http://www.google.com".parse().unwrap())
        // Map the _streaming_ response from google into a _streaming_
        // response from us
        .map(|res| Response::build().body(res.body()))
        // Use `.into_box` to turn this future stream into a `BoxFuture`
        // that can be easily returned on stable Rust.
        //
        // When `impl Trait` becomes available on stable Rust, this
        // necessity will go away
        .into_box()
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Info).unwrap();
    // Our simple HTTP proxy doesn't need a Router and Shio doesn't force
    // a router on you.

    // `Shio::new` expects a single root handler. By default this
    // is a (wrapped) Router. As we don't need a router, we are using
    // `Shio::new` to specify our own root handler.

    info!("Initialising the proxy.");
    Shio::new(proxy).run(":7878").unwrap();

    // Here is an example of what `Shio::default` is equivalent to:
    /*

    Shio::new(
      // Stack is a middleware container that executes each middleware
      // around _its_ root handler
      shio::Stack::new(shio::router::Router::new())
        // By default, the Recover middleware is included in your stack
        .with(shio::middleware::Recover)
    )

    */
}
