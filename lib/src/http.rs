//! Types that map directly to concepts in HTTP.
//!
//! This module exports types that map to HTTP concepts or to the underlying HTTP library
//! when needed.
#![allow(non_upper_case_globals)]

use hyper;
use http_types;

// NOTE: This facade is only around to not completely break current usage of 0.2.
//       It will be removed in 0.4.

/// The Request Method (VERB).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Method(http_types::Method);

impl Method {
    pub const HEAD: Method = Method(http_types::Method::HEAD);
    pub const OPTIONS: Method = Method(http_types::Method::OPTIONS);
    pub const GET: Method = Method(http_types::Method::GET);
    pub const POST: Method = Method(http_types::Method::POST);
    pub const PUT: Method = Method(http_types::Method::PUT);
    pub const PATCH: Method = Method(http_types::Method::PATCH);
    pub const DELETE: Method = Method(http_types::Method::DELETE);

    #[deprecated(since = "0.2.0", note = "use `Method::HEAD` instead")]
    pub const Head: Method = Method(http_types::Method::HEAD);

    #[deprecated(since = "0.2.0", note = "use `Method::OPTIONS` instead")]
    pub const Options: Method = Method(http_types::Method::OPTIONS);

    #[deprecated(since = "0.2.0", note = "use `Method::GET` instead")]
    pub const Get: Method = Method(http_types::Method::GET);

    #[deprecated(since = "0.2.0", note = "use `Method::POST` instead")]
    pub const Post: Method = Method(http_types::Method::POST);

    #[deprecated(since = "0.2.0", note = "use `Method::PUT` instead")]
    pub const Put: Method = Method(http_types::Method::PUT);

    #[deprecated(since = "0.2.0", note = "use `Method::PATCH` instead")]
    pub const Patch: Method = Method(http_types::Method::PATCH);

    #[deprecated(since = "0.2.0", note = "use `Method::DELETE` instead")]
    pub const Delete: Method = Method(http_types::Method::DELETE);

    // Temporary until Hyper 0.12
    pub(crate) fn to_hyper_method(&self) -> hyper::Method {
        match *self {
            Self::OPTIONS => hyper::Method::Options,
            Self::HEAD => hyper::Method::Head,
            Self::GET => hyper::Method::Get,
            Self::POST => hyper::Method::Post,
            Self::PUT => hyper::Method::Put,
            Self::PATCH => hyper::Method::Patch,
            Self::DELETE => hyper::Method::Delete,

            _ => unimplemented!(),
        }
    }

    // Temporary until Hyper 0.12
    pub(crate) fn from_hyper_method(hm: &hyper::Method) -> Self {
        match *hm {
            hyper::Method::Options => Self::OPTIONS,
            hyper::Method::Head => Self::HEAD,
            hyper::Method::Get => Self::GET,
            hyper::Method::Post => Self::POST,
            hyper::Method::Put => Self::PUT,
            hyper::Method::Patch => Self::PATCH,
            hyper::Method::Delete => Self::DELETE,

            _ => unimplemented!(),
        }
    }
}

// HTTP Headers
pub use hyper::header;

// HTTP Status Codes
pub use hyper::StatusCode;
