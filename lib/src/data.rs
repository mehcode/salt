use hyper;
use futures::{Poll, Stream};

use errors::Error;

#[derive(Default)]
pub struct Data(hyper::Body);

impl Data {
    pub(crate) fn new(body: hyper::Body) -> Self {
        Data(body)
    }
}

impl Stream for Data {
    type Item = hyper::Chunk;
    type Error = Error;

    #[inline]
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll().map_err(Error::from)
    }
}

impl Into<hyper::Body> for Data {
    fn into(self) -> hyper::Body {
        self.0
    }
}