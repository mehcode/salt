use std::sync::Arc;
use std::fmt;
use std::panic::AssertUnwindSafe;

use hyper;
use tokio_core::reactor::Handle;
use futures::{future, Future, IntoFuture};
use unsafe_any::UnsafeAny;

use request::Request;
use handler::{default_catch, Handler};
use context::Context;
use state::State;
use util::typemap::TypeMap;
use ext::BoxFuture;
use Data;
use http::Method;

// FIXME: Why does #[derive(Clone)] not work here? This _seems_ like a implementation that
//        should be auto-derived.

// #[derive(Clone)]
pub(crate) struct Service<H: Handler + 'static>
where
    <H::Result as IntoFuture>::Error: fmt::Debug + Send,
{
    handler: Arc<H>,
    handle: Handle,
    shared_state: Arc<TypeMap<UnsafeAny + Send + Sync>>,
}

impl<H: Handler + 'static> Service<H>
where
    <H::Result as IntoFuture>::Error: fmt::Debug + Send,
{
    pub(crate) fn new(
        handler: Arc<H>,
        handle: Handle,
        shared_state: Arc<TypeMap<UnsafeAny + Send + Sync>>,
    ) -> Self {
        Self {
            handler,
            handle,
            shared_state,
        }
    }
}

impl<H: Handler + 'static> Clone for Service<H>
where
    <H::Result as IntoFuture>::Error: fmt::Debug + Send,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            handle: self.handle.clone(),
            shared_state: self.shared_state.clone(),
        }
    }
}

pub(crate) fn from_hyper_request(request: hyper::Request) -> (Request, Data) {
    let (method, uri, version, header, body) = request.deconstruct();
    (
        Request::new((Method::from_hyper_method(&method), uri, version, header)),
        Data::new(body),
    )
}

impl<H: Handler + 'static> hyper::server::Service for Service<H>
where
    <H::Result as IntoFuture>::Error: fmt::Debug + Send,
{
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        let (request, data) = from_hyper_request(request);
        let state = State::new(self.shared_state.clone());
        let ctx = Context::new(self.handle.clone(), request, state, data);
        let handler = self.handler.clone();

        Box::new(
            AssertUnwindSafe(future::lazy(move || handler.call(ctx).into_future()))
                .catch_unwind()
                .then(|result| -> BoxFuture<hyper::Response, hyper::Error> {
                    Box::new(future::ok(
                        (match result {
                            Err(err) => default_catch(err),
                            Ok(Err(err)) => default_catch(err),
                            Ok(Ok(response)) => response,
                        }).into_hyper_response(),
                    ))
                }),
        )
    }
}
