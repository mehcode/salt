extern crate shio;

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use shio::prelude::*;

#[derive(Default)]
struct HandlerWithState {
    counter: AtomicUsize,
}

impl shio::Handler for HandlerWithState {
    type Result = Response;

    fn call(&self, _: Context) -> Self::Result {
        let counter = self.counter.fetch_add(1, Ordering::Relaxed);

        Response::with(format!(
            "Hi, #{} (from thread: {:?})\n",
            counter,
            thread::current().id()
        ))
    }
}

fn main() {
    Shio::default()
        .route((Method::GET, "/", HandlerWithState::default()))
        .run(":7878")
        .unwrap();
}
