use hyper::Body;

use response::Response;
use http::StatusCode;
use http::header::Header;

/// An HTTP response builder.
///
/// This type can be used to construct a [`Response`] through a builder-like pattern.
///
/// This builder may be finalized into a [`Response`] through either `Builder::body` or
/// `Builder::into`.
///
/// ```rust
/// # use shio::response::{self, Response};
/// # use shio::StatusCode;
/// // A 204, "No Content", Response
/// let response: Response = response::Builder::new().status(StatusCode::NoContent).into();
///
/// // A "Hello World" Response
/// let response: Response = response::Builder::new().body("Hello World\n");
/// ```
///
/// [`Response`]: struct.Response.html
#[derive(Default)]
pub struct Builder {
    inner: Response,
}

impl Builder {
    /// Creates a new default instance of `Builder` to construct a [`Response`].
    ///
    /// [`Response`]: struct.Response.html
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the HTTP status for this response.
    ///
    /// ```rust
    /// # use shio::response::{self, Response};
    /// # use shio::StatusCode;
    /// let response: Response = response::Builder::new().status(StatusCode::BadRequest).into();
    /// ```
    #[inline]
    pub fn status(mut self, status: StatusCode) -> Self {
        self.inner.set_status(status);
        self
    }

    /// Appends a [`Header`] to this response.
    ///
    /// ```rust
    /// # use shio::response::{self, Response};
    /// # use shio::header;
    /// let response: Response = response::Builder::new()
    ///     // Date: Tue, 15 Nov 1994 08:12:31 GMT
    ///     .header(header::Date(std::time::SystemTime::now().into()))
    ///     // Content-Length: 30
    ///     .header(header::ContentLength(30))
    ///     // Server: Shio/0.0.4
    ///     .header(header::Server::new("Shio/0.0.4"))
    ///     .into();
    /// ```
    ///
    /// [`Header`]: ../trait.Header.html
    #[inline]
    pub fn header<H: Header>(mut self, header: H) -> Self {
        self.inner.headers_mut().set(header);
        self
    }

    /// Consumes this builder, using the provided body to return a constructed [`Response`].
    ///
    /// ```rust
    /// # use shio::response::{self, Response};
    /// let response: Response = response::Builder::new().body("Hello World\n");
    /// ```
    ///
    /// [`Response`]: struct.Response.html
    #[inline]
    pub fn body<B: Into<Body>>(mut self, body: B) -> Response {
        self.inner.set_body(body);
        self.inner
    }
}

impl From<Builder> for Response {
    fn from(builder: Builder) -> Self {
        builder.inner
    }
}
