//! The `Format` struct and its implementations.

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, string::String};

/// Various well-known formats, along with the possibility for a custom format
/// (provided either at compile-time or runtime).
#[allow(clippy::missing_docs_in_private_items)] // variants
#[cfg_attr(__time_02_supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    #[cfg_attr(__time_02_docs, doc(alias = "ISO8601"))]
    Rfc3339,
    Custom(String),
    #[cfg(not(__time_02_supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

// TODO We're only using `AsRef` for back-compatibility. In 0.3, switch this to
// `Into<Cow<'a, str>>`, which is both broader and avoids unnecessary clones.
// This will require the addition of a lifetime to the `Format` struct.

impl<T: AsRef<str>> From<T> for Format {
    fn from(s: T) -> Self {
        Format::Custom(s.as_ref().to_owned())
    }
}
