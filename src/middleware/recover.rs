//! Defines the recover middleware, that permit catch `panic!`
//! and return an internal error

use handler::BoxHandler;
use context::Context;
use response::{Response, BoxFutureResponse};

use std::panic::AssertUnwindSafe;
use futures::{lazy, future, Future};
use hyper::StatusCode;

/// Catch `panic!`, and send back an error 500.
pub fn recover_panics(next: BoxHandler) -> BoxHandler {
    Box::new(move |ctx: Context| -> BoxFutureResponse {
        Box::new(
            AssertUnwindSafe(lazy(move || {
                next.call(ctx)
            }))
            .catch_unwind()
            .then(move |result| -> BoxFutureResponse {
                Box::new(match result {
                    Err(_err) => {
                        // a panic occurs, send back an InternalServerError response
                        future::ok(Response::build().status(StatusCode::InternalServerError).into())
                    },
                    Ok(Err(e)) => {
                        future::err(e) 
                    },
                    Ok(Ok(x))  => { 
                        future::ok(x) 
                    },
                })
            })
        )
    })
}