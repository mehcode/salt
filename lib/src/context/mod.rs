mod builder;

use std::ops::Deref;

use tokio_core::reactor::Handle;
use unsafe_any::UnsafeAny;

use util::typemap::TypeMap;
use request::Request;
use state::{FromState, State};
use Data;
pub use state::Key;

pub use self::builder::Builder;

/// `Context` represents the context of the current HTTP request.
///
/// A `Context` consists of:
///     - The current HTTP [`Request`].
///     - Shared and per-request [`State`].
///     - A [`Handle`] referencing the event loop in which this request is being
///       handled.
///
/// [`Handle`]: https://docs.rs/tokio-core/0.1/tokio_core/reactor/struct.Handle.html
/// [`Request`]: ../request/struct.Request.html
/// [`State`]: ../struct.State.html
pub struct Context {
    state: State,
    handle: Handle,
    request: Request,
    body: Data,
}

impl Context {
    pub(crate) fn new(handle: Handle, request: Request, state: State, body: Data) -> Self {
        Self {
            handle,
            request,
            state,
            body,
        }
    }

    /// Return a reference to a handle to the event loop this `Context` is associated with.
    #[inline]
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Take the request body.
    pub fn data(self) -> Data {
        self.body
    }

    /// Puts a value into the request state.
    pub fn put<K: Key>(&mut self, value: K::Value) {
        self.state.put::<K>(value);
    }

    /// Gets a value from the request state.
    ///
    /// With the `nightly` feature enabled, this will fallback to checking the shared
    /// state.
    ///
    /// # Panics
    ///
    /// If there is no value in the request state of the given type.
    pub fn get<T: FromState>(&self) -> &T::Value {
        self.state.get::<T>()
    }

    /// Gets a value from the request state.
    pub fn try_get<T: FromState>(&self) -> Option<&T::Value> {
        self.state.try_get::<T>()
    }

    /// Gets a reference to the shared state.
    pub fn shared(&self) -> &TypeMap<UnsafeAny + Send + Sync> {
        self.state.shared()
    }

    /// Deconstruct current context
    pub fn deconstruct(self) -> (Handle, State, Request, Data) {
        (self.handle, self.state, self.request, self.body)
    }
    
    /// Create a new context [`Builder`]
    ///
    /// [`Builder`]: struct.Builder.html
    pub fn builder(handle: Handle) -> Builder {
        Builder::new(handle)
    }
}

impl Deref for Context {
    type Target = Request;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}
