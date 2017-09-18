use std::sync::Arc;

use unsafe_any::UnsafeAny;

use util::typemap::TypeMap;
pub use util::typemap::Key;

pub struct State {
    /// State local to a specific request.
    request: TypeMap,

    /// State shared across all requests.
    pub(crate) shared: Arc<TypeMap<UnsafeAny + Send + Sync>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            request: TypeMap::new(),
            shared: Arc::new(TypeMap::custom()),
        }
    }
}

impl State {
    pub(crate) fn new(shared: Arc<TypeMap<UnsafeAny + Send + Sync>>) -> Self {
        Self {
            request: TypeMap::new(),
            shared,
        }
    }

    /// Puts a value into the request state.
    pub fn put<K: Key>(&mut self, value: K::Value) {
        self.request.put::<K>(value);
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
        // FIXME: Don't use unwrap and explain the error
        T::from_state(self).unwrap()
    }

    /// Gets a value from the request state.
    pub fn try_get<T: FromState>(&self) -> Option<&T::Value> {
        T::from_state(self)
    }

    /// Gets a reference to the shared state.
    pub fn shared(&self) -> &TypeMap<UnsafeAny + Send + Sync> {
        &*self.shared
    }
}

#[doc(hidden)]
pub trait FromState: Key {
    fn from_state(state: &State) -> Option<&Self::Value>;
}

#[cfg(not(feature = "nightly"))]
impl<T: Key> FromState for T {
    fn from_state(state: &State) -> Option<&Self::Value> {
        state.request.try_get::<T>()
    }
}

#[cfg(feature = "nightly")]
impl<T: Key> FromState for T {
    default fn from_state(state: &State) -> Option<&Self::Value> {
        state.request.try_get::<T>()
    }
}

#[cfg(feature = "nightly")]
impl<T: Key> FromState for T
where
    <T as Key>::Value: Send + Sync,
{
    default fn from_state(state: &State) -> &<T as Key>::Value {
        state
            .shared
            .try_get::<T>()
            .or_else(|| state.request.try_get::<T>())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::*;

    struct Number;

    impl Key for Number {
        type Value = u64;
    }

    struct RcNumber;

    impl Key for RcNumber {
        type Value = Rc<u64>;
    }

    #[test]
    fn test_state_request() {
        let mut state = State::default();
        state.put::<Number>(100);

        assert_eq!(state.get::<Number>(), &100);
    }

    #[test]
    fn test_state_request_non_sync() {
        let rc_num = Rc::new(320);

        let mut state = State::default();
        state.put::<RcNumber>(rc_num.clone());

        assert_eq!(state.get::<RcNumber>(), &rc_num);
    }

    #[test]
    #[cfg(feature = "nightly")]
    fn test_state_shared_fallback() {
        let mut shared = TypeMap::<UnsafeAny + Send + Sync>::custom();
        shared.insert::<Number>(7878);

        let state = State::new(Arc::new(shared));

        assert_eq!(state.get::<Number>(), &7878);
    }
}
