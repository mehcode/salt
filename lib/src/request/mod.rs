use std::net::SocketAddr;
use chrono::offset::Local;
use hyper::{self, Method};

pub struct Request {
    method: Method,
    uri: hyper::Uri,
    version: hyper::HttpVersion,
    headers: hyper::Headers,
    remote_addr: Option<SocketAddr>,
}

impl Request {
    pub(crate) fn new(
        components: (Method, hyper::Uri, hyper::HttpVersion, hyper::Headers, Option<SocketAddr>),
    ) -> Self {
        Self {
            method: components.0,
            uri: components.1,
            version: components.2,
            headers: components.3,
            remote_addr: components.4,
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

    pub fn log_line(&self) -> String {
        if let Some(remote_addr) = self.remote_addr {
            format!("{} {} {} {}", remote_addr, Local::now(), self.method, self.path())
        } else {
            format!("{} {} {}", Local::now(), self.method, self.path())
        }
    }
}
