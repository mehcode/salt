//! Proxy google and stream the response back to the client.
//! This could be expanded into a simple http-proxy.

// TODO: Expand this a bit.
//  - Add some command line args: `proxy -p 7878 http://www.google.com`
//  - Add log and log when a request is proxied
//  - Use `Shio::new( .. )` as the router is just getting in the way here
//  - Proxy full request path

extern crate hyper;
extern crate shio;
extern crate clap;

use shio::prelude::*;
use hyper::Client;
use clap::{App, Arg};

fn proxy(ctx: Context) -> BoxFuture<Response, hyper::Error> {
    // Additional work can be scheduled on the thread-local event loop,
    // as each handler receives a reference to it
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
    let matches = App::new("Proxy")
                    .version(env!("CARGO_PKG_VERSION"))
                    .arg(Arg::with_name("port")
                        .short("p")
                        .value_name("PORT")
                        .help("Sets port number"))
                    .get_matches();
    let port = matches.value_of("port").unwrap_or("7878");
    // Our simple HTTP proxy doesn't need a Router and Shio doesn't force
    // a router on you.

    // `Shio::new` expects a single root handler. By default this
    // is a (wrapped) Router. As we don't need a router, we are using
    // `Shio::new` to specify our own root handler.

    Shio::new(proxy).run(format!(":{}", port)).unwrap();

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
