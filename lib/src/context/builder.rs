
use tokio_core::reactor::Handle;
use std::sync::Arc;
use unsafe_any::UnsafeAny;
use hyper::{Method, error, HttpVersion, Body};
use hyper::header::Header;

use super::Context;
use state::State;
use util::typemap::TypeMap;
use request;
use data;

/// Helper struct for construct a [`Context`]
///
/// [`Context`]: struct.Context.html
pub struct Builder {
    handle: Handle,

    state: State,

    // for request
    request: request::Builder,
    body: Body,
}

impl Builder {
    /// Create a new `Builder` from a tokio_core `Handle`
    pub fn new(handle: Handle) -> Self {
        Self {
            handle,
            state: State::default(),
            request: request::Builder::default(),
            body: Body::default(),
        }
    }

    /// Set the shared data.
    pub fn shared(mut self, shared: Arc<TypeMap<UnsafeAny + Send + Sync>>) -> Self {
        self.state.shared = shared;
        self
    }

    /// Set the HTTP method to `method`
    pub fn method(mut self, method: Method) -> Self {
        self.request = self.request.method(method);
        self
    }

    /// Set the uri
    pub fn uri(mut self, uri: &str) -> Self {
        self.request = self.request.uri(uri);
        self
    }

    /// Set the HTTP version
    pub fn version(mut self, ver: HttpVersion) -> Self {
        self.request = self.request.version(ver);
        self
    }

    /// Set an header
    pub fn set_header<H: Header>(mut self, value: H) -> Self {
        self.request = self.request.set_header(value);
        self
    }

    /// Set the request data
    pub fn set_data<B: Into<Body>>(mut self, body: B) -> Self {
        self.body = body.into();
        self
    }

    /// Create the `Context`, returning any error that occurs during build.
    pub fn finalize(self) -> Result<Context, error::UriError> {
        let Self {
            handle, state,
            request,
            body
        } = self;

        Ok(Context::new(handle, request.finalize()?, state, data::Data::new(body)))
    }
}

