pub use crate::format::parse::Error as Parse;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::fmt;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact
/// error returned. `Result<_, time::Error>` will work in these situations.
// Boxing the `ComponentRange` reduces the size of `Error` from 72 bytes to 16.
// TODO(0.3) Zero-sized structs can be eliminated in favor of a simple variant.
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[cfg_attr(__time_02_supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    ConversionRange(ConversionRange),
    ComponentRange(Box<ComponentRange>),
    Parse(Parse),
    IndeterminateOffset(IndeterminateOffset),
    Format(Format),
    #[cfg(not(__time_02_supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConversionRange(e) => e.fmt(f),
            Error::ComponentRange(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            Error::IndeterminateOffset(e) => e.fmt(f),
            Error::Format(e) => e.fmt(f),
            #[cfg(not(__time_02_supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ConversionRange(err) => Some(err),
            Error::ComponentRange(box_err) => Some(box_err.as_ref()),
            Error::Parse(err) => Some(err),
            Error::IndeterminateOffset(err) => Some(err),
            Error::Format(err) => Some(err),
            #[cfg(not(__time_02_supports_non_exhaustive))]
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

/// An error type indicating that a conversion failed because the target type
/// could not store the initial value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRange;

impl fmt::Display for ConversionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Source value is out of range for the target type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionRange {}

impl From<ConversionRange> for Error {
    fn from(original: ConversionRange) -> Self {
        Error::ConversionRange(original)
    }
}

/// An error type indicating that a component provided to a method was out of
/// range, causing a failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need
// for a type parameter.
#[cfg_attr(__time_02_supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub name: &'static str,
    /// Minimum allowed value, inclusive.
    pub minimum: i64,
    /// Maximum allowed value, inclusive.
    pub maximum: i64,
    /// Value that was provided.
    pub value: i64,
    /// The minimum and/or maximum value is conditional on the value of other
    /// parameters.
    pub conditional_range: bool,
    #[cfg(not(__time_02_supports_non_exhaustive))]
    #[doc(hidden)]
    pub(crate) __non_exhaustive: (),
}

impl fmt::Display for ComponentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be in the range {}..={}",
            self.name, self.minimum, self.maximum
        )?;

        if self.conditional_range {
            write!(f, ", given values of other parameters")?;
        }

        Ok(())
    }
}

impl From<ComponentRange> for Error {
    fn from(original: ComponentRange) -> Self {
        Error::ComponentRange(Box::new(original))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ComponentRange {}

impl From<Parse> for Error {
    fn from(original: Parse) -> Self {
        Error::Parse(original)
    }
}

/// The system's UTC offset could not be determined at the given datetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateOffset;

impl fmt::Display for IndeterminateOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IndeterminateOffset {}

impl From<IndeterminateOffset> for Error {
    fn from(original: IndeterminateOffset) -> Self {
        Error::IndeterminateOffset(original)
    }
}

/// An error occurred while formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Format {
    /// The format provided requires more information than the type provides.
    InsufficientTypeInformation,
    /// An error occurred while formatting into the provided stream.
    StdFmtError,
    #[cfg(not(__time_02_supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::InsufficientTypeInformation => {
                f.write_str("The format provided requires more information than the type provides.")
            }
            Format::StdFmtError => fmt::Error.fmt(f),
            #[cfg(not(__time_02_supports_non_exhaustive))]
            Format::__NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Format {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Format::StdFmtError => Some(&fmt::Error),
            _ => None,
        }
    }
}

// This is necessary to be able to use `?` with various formatters.
impl From<fmt::Error> for Format {
    fn from(_: fmt::Error) -> Self {
        Format::StdFmtError
    }
}

impl From<Format> for Error {
    fn from(error: Format) -> Self {
        Error::Format(error)
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use super::*;
    use std::error::Error as ParseError;

    #[test]
    fn indeterminate_offset() {
        assert_eq!(
            IndeterminateOffset.to_string(),
            Error::IndeterminateOffset(IndeterminateOffset).to_string()
        );
        assert!(match Error::from(IndeterminateOffset).source() {
            Some(error) => error.is::<IndeterminateOffset>(),
            None => false,
        });
    }
}
