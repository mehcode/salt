extern crate shio;

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use shio::prelude::*;
use shio::context::Key;

pub struct SharedCounter;

impl Key for SharedCounter {
    type Value = AtomicUsize;
}

fn hello(context: Context) -> Response {
    let counter = context.shared().get::<SharedCounter>().fetch_add(1, Ordering::Relaxed);

    Response::with(format!(
        "Hi, #{} (from thread: {:?})\n",
        counter,
        thread::current().id()
    ))
}

fn main() {
    Shio::default()
        .manage::<SharedCounter>(AtomicUsize::default())
        .route((Method::GET, "/", hello))
        .run(":7878")
        .unwrap();
}
