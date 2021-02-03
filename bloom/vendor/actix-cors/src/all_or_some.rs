/// An enum signifying that some of type `T` is allowed, or `All` (anything is allowed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AllOrSome<T> {
    /// Everything is allowed. Usually equivalent to the `*` value.
    All,

    /// Only some of `T` is allowed
    Some(T),
}

/// Default as `AllOrSome::All`.
impl<T> Default for AllOrSome<T> {
    fn default() -> Self {
        AllOrSome::All
    }
}

impl<T> AllOrSome<T> {
    /// Returns whether this is an `All` variant.
    pub fn is_all(&self) -> bool {
        matches!(self, AllOrSome::All)
    }

    /// Returns whether this is a `Some` variant.
    #[allow(dead_code)]
    pub fn is_some(&self) -> bool {
        !self.is_all()
    }

    /// Provides a shared reference to `T` if variant is `Some`.
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            AllOrSome::All => None,
            AllOrSome::Some(ref t) => Some(t),
        }
    }

    /// Provides a mutable reference to `T` if variant is `Some`.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match *self {
            AllOrSome::All => None,
            AllOrSome::Some(ref mut t) => Some(t),
        }
    }
}

#[cfg(test)]
#[test]
fn tests() {
    assert!(AllOrSome::<()>::All.is_all());
    assert!(!AllOrSome::<()>::All.is_some());

    assert!(!AllOrSome::Some(()).is_all());
    assert!(AllOrSome::Some(()).is_some());
}
