//! Types representing errors that can occur from inside Shio.

use std::fmt;
use std::error::Error as StdError;
use std::io;

use hyper;

/// An error that occurs during `Shio::listen` or `Shio::run`.
#[derive(Debug)]
pub struct ListenError {
    inner: ListenErrorKind,
}

impl fmt::Display for ListenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ListenErrorKind::Io(ref err) => err.fmt(f),
        }
    }
}

impl StdError for ListenError {
    fn description(&self) -> &str {
        match self.inner {
            ListenErrorKind::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self.inner {
            ListenErrorKind::Io(ref err) => err.cause(),
        }
    }
}

impl<T: Into<ListenErrorKind>> From<T> for ListenError {
    fn from(err: T) -> Self {
        Self { inner: err.into() }
    }
}

#[derive(Debug)]
pub enum ListenErrorKind {
    Io(io::Error),
}

impl From<io::Error> for ListenErrorKind {
    fn from(err: io::Error) -> Self {
        ListenErrorKind::Io(err)
    }
}

/// A generic "error" that can occur from inside Shio.
#[derive(Debug)]
pub struct Error { inner: ErrorKind }

#[derive(Debug)]
enum ErrorKind {
    Listen(ListenError),
    Hyper(hyper::Error),
}

impl From<ListenError> for Error {
    fn from(err: ListenError) -> Self {
        Self { inner: ErrorKind::Listen(err) }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self { inner: ErrorKind::Hyper(err) }
    }
}

impl From<hyper::error::UriError> for Error {
    fn from(err: hyper::error::UriError) -> Self {
        Self { inner: ErrorKind::Hyper(err.into()) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            ErrorKind::Hyper(ref err) => err.fmt(f),
            ErrorKind::Listen(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.inner {
            ErrorKind::Hyper(ref err) => err.description(),
            ErrorKind::Listen(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self.inner {
            ErrorKind::Hyper(ref err) => err.cause(),
            ErrorKind::Listen(ref err) => err.cause(),
        }
    }
}
