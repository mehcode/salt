#![feature(proc_macro)]

extern crate shio;

use shio::prelude::*;

// Simple requests should be simple, even in the face of asynchronous design.
#[get("/")]
fn hello_world(_: Context) -> Response {
    Response::with("Hello World!\n")
}

#[get("/{name}")]
fn hello(ctx: Context) -> Response {
    Response::with(format!("Hello, {}!", &ctx.get::<Parameters>()["name"]))
}

fn main() {
    Shio::default()
        .route(hello_world)
        .route(hello)
        .run(":7878")
        .unwrap();
}
