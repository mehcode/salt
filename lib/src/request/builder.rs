
use hyper::{Method, Uri, error, HttpVersion, Headers};
use hyper::header::Header;

use super::Request;
use errors::Error;

/// Helper struct for construct a [`Request`]
///
/// [`Request`]: struct.Context.html
pub struct Builder {
    method: Method,
    uri: Result<Uri, error::UriError>,
    version: HttpVersion,
    headers: Headers,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            method: Method::Get,
            uri: "/".parse::<Uri>(),
            version: HttpVersion::Http11,
            headers: Headers::new(),
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
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
    pub fn header<H: Header>(mut self, value: H) -> Self {
        self.headers.set(value);
        self
    }

    /// Create the `Context`, returning any error that occurs during build.
    pub fn finalize(self) -> Result<Request, Error> {
        let Self {
            method, uri,
            version, headers,
        } = self;

        let uri = uri?;

        Ok(Request::new((method, uri, version, headers)))
    }
}

