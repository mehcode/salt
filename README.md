# Shio
![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Build Status](https://travis-ci.org/mehcode/shio-rs.svg?branch=master)](https://travis-ci.org/mehcode/shio-rs)
[![Coverage Status](https://coveralls.io/repos/github/mehcode/shio-rs/badge.svg?branch=master)](https://coveralls.io/github/mehcode/shio-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/shio.svg)](https://crates.io/crates/shio)
[![Crates.io](https://img.shields.io/crates/d/shio.svg)](https://crates.io/crates/shio)
[![Docs.rs](https://docs.rs/shio/badge.svg)](https://docs.rs/shio)
[![IRC](https://img.shields.io/badge/chat-%23shio-yellow.svg)](https://kiwiirc.com/client/irc.mozilla.org/#shio)
> Shio is a fast, simple, and asynchronous micro web-framework for Rust.

 - **Asynchronous**. Handlers are both handled _asynchronously_ and may be _asynchronous_ themselves. A `shio::Handler` receives a `tokio_core::reactor::Handle` which may be used to schedule additional work on the thread-local event loop.

 - **Multithreaded**. By default, requests are handled by multiple threads, each running an event loop powered by `tokio`.

 - **Stability**. Shio is fully committed to working and continuing to work on **stable** Rust.

## Usage

```toml
[dependencies]
shio = "0.2.0"
```

```rust
extern crate shio;

use shio::prelude::*;

fn hello_world(_: Context) -> Response {
    Response::with("Hello World!\n")
}

fn hello(ctx: Context) -> Response {
    Response::with(format!("Hello, {}!\n", &ctx.get::<Parameters>()["name"]))
}

fn main() {
    Shio::default()
        .route((Method::GET, "/", hello_world))
        .route((Method::GET, "/{name}", hello))
        .run(":7878").unwrap();
}
```

## Examples

### [Stateful](examples/stateful/src/main.rs)

A request handler is a value that implements the `shio::Handler` trait.

Handlers are **not** cloned on each request and therefore may contain state.
Note that any fields must be `Send + Sync`.

```rust
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
```

### Even More Examples

Many more usage [examples/](https://github.com/mehcode/shio-rs/tree/master/examples) are included with Shio.

  - [Render templates](examples/templates_askama/src/main.rs) with [Askama](https://github.com/djc/askama)
  - [Stream a HTTP request](examples/proxy/src/main.rs) with [Hyper](https://github.com/hyperium/hyper)
  - [Postgres](examples/postgres/src/main.rs) with [tokio-postgres](https://github.com/sfackler/rust-postgres)

Examples may be ran with `cargo run -p <example name>`.
For instance, to run the `hello` example, use:

```bash
$ cargo run -p hello
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
