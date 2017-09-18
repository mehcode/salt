#![cfg_attr(feature = "cargo-clippy", warn(clippy, clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(missing_docs_in_private_items, stutter))]
#![cfg_attr(feature = "nightly", feature(specialization))]

extern crate futures;
extern crate http as http_types;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate net2;
extern crate num_cpus;
extern crate regex;
extern crate tokio_core;
extern crate unsafe_any;

pub mod state;
pub mod context;
mod handler;
mod shio;
mod service;
pub mod ext;
pub mod response;
pub mod request;
pub mod errors;
pub mod router;
pub mod util;
pub mod data;
pub mod http;

pub use response::Response;
pub use request::Request;
pub use shio::Shio;
pub use context::Context;
pub use state::State;
pub use handler::Handler;
pub use data::Data;
pub use errors::Error;

/// Re-exports important traits and types. Meant to be glob imported when using Shio.
pub mod prelude {
    pub use {http, Context, Request, Response, Shio};
    pub use router::Parameters;
    pub use ext::{BoxFuture, FutureExt};
    pub use http::{Method, StatusCode};

    pub use futures::{Future, IntoFuture, Stream};
}
