
use tokio_core::reactor::Handle;
use std::sync::Arc;
use unsafe_any::UnsafeAny;
use hyper::{Method, Uri, error, HttpVersion, Headers, Body};
use hyper::header::Header;

use super::Context;
use state::State;
use util::typemap::TypeMap;
use request::Request;
use data;

/// Helper struct for construct a [`Context`]
///
/// [`Context`]: struct.Context.html
pub struct Builder {
    handle: Handle,

    state: State,

    // for request
    method: Method,
    uri: Result<Uri, error::UriError>,
    version: HttpVersion,
    headers: Headers,
    body: Body,
}

impl Builder {
    /// Create a new `ContextBuilder` from a tokio_core `Handle`
    pub fn new(handle: Handle) -> Self {
        Self {
            handle,
            state: State::default(),
            method: Method::Get,
            uri: "/".parse::<Uri>(),
            version: HttpVersion::Http11,
            headers: Headers::new(),
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
        self.method = method;
        self
    }

    /// Set the uri
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.parse::<Uri>();
        self
    }

    /// Set the HTTP version
    pub fn version(mut self, ver: HttpVersion) -> Self {
        self.version = ver;
        self
    }

    /// Set an header
    pub fn set_header<H: Header>(mut self, value: H) -> Self {
        self.headers.set(value);
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
            method, uri,
            version, headers,
            body
        } = self;

        let uri = uri?;

        let request = Request::new((method, uri, version, headers));
        let context = Context::new(handle, request, state, data::Data::new(body));
        Ok(context)
    }
}

