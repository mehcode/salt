mod builder;
mod responder;

pub use self::builder::Builder;
pub use self::responder::Responder;

use std::fmt;

use futures::{Future, IntoFuture};
use futures::future::{self, FutureResult};
use hyper;

use http::StatusCode;
use http::header::Headers;

/// Represents an HTTP response.
pub struct Response {
    inner: hyper::Response<hyper::Body>,
}

impl Response {
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new builder-style object to manufacture a Response.
    ///
    pub fn build() -> Builder {
        Default::default()
    }

    pub fn with<R: Responder>(responder: R) -> R::Result
    where
        <R::Result as IntoFuture>::Error: fmt::Debug + Send + 'static,
    {
        responder.to_response()
    }

    pub(crate) fn into_hyper_response(self) -> hyper::Response<hyper::Body> {
        self.inner
    }

    /// Get the status.
    #[inline]
    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    /// Set the [`StatusCode`] for this response.
    ///
    /// [`StatusCode`]: ../enum.StatusCode.html
    #[inline]
    pub fn set_status(&mut self, status: StatusCode) {
        self.inner.set_status(status);
    }

    /// Take the body.
    pub fn body(self) -> hyper::Body {
        self.inner.body()
    }

    /// Set the body for this response.
    #[inline]
    pub fn set_body<B: Into<hyper::Body>>(&mut self, body: B) {
        self.inner.set_body(body.into());
    }

    /// Get a mutable reference to the headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers {
        self.inner.headers_mut()
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            inner: hyper::Response::new(),
        }
    }
}

impl IntoFuture for Response {
    type Item = Self;
    type Error = hyper::Error;
    type Future = FutureResult<Self::Item, Self::Error>;

    #[inline]
    fn into_future(self) -> Self::Future {
        future::ok(self)
    }
}

#[deprecated(since = "0.0.6", note = "use BoxFuture<Response, _> instead")]
pub type BoxFutureResponse<E = hyper::Error> = Box<Future<Item = Response, Error = E>>;
