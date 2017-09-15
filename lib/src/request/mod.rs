mod builder;

use hyper::{self, Method};

pub use self::builder::Builder;

pub struct Request {
    method: Method,
    uri: hyper::Uri,
    version: hyper::HttpVersion,
    headers: hyper::Headers,
}

impl Request {
    pub(crate) fn new(
        components: (Method, hyper::Uri, hyper::HttpVersion, hyper::Headers),
    ) -> Self {
        Self {
            method: components.0,
            uri: components.1,
            version: components.2,
            headers: components.3,
        }
    }

    /// Returns a reference to the request HTTP version.
    #[inline]
    pub fn version(&self) -> &hyper::HttpVersion {
        &self.version
    }

    /// Returns a reference to the request headers.
    #[inline]
    pub fn headers(&self) -> &hyper::Headers {
        &self.headers
    }

    /// Returns a reference to the request HTTP method.
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns a reference to the request URI.
    #[inline]
    pub fn uri(&self) -> &hyper::Uri {
        &self.uri
    }

    /// Returns a reference to the request path.
    #[inline]
    pub fn path(&self) -> &str {
        self.uri.path()
    }

    /// Create a new `Builder`
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }
}
