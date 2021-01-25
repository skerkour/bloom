use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Raised if a project ID cannot be parsed from a string.
#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseProjectIdError {
    /// Raised if the value is not an integer in the supported range.
    #[error("invalid value for project id")]
    InvalidValue,
    /// Raised if an empty value is parsed.
    #[error("empty or missing project id")]
    EmptyValue,
}

/// Represents a project ID.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct ProjectId(u64);

impl ProjectId {
    /// Creates a new project ID from its numeric value.
    #[inline]
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the numeric value of this project id.
    #[inline]
    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

macro_rules! impl_from {
    ($ty:ty) => {
        impl From<$ty> for ProjectId {
            #[inline]
            fn from(val: $ty) -> Self {
                Self::new(val as u64)
            }
        }
    };
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);

macro_rules! impl_try_from {
    ($ty:ty) => {
        impl TryFrom<$ty> for ProjectId {
            type Error = ParseProjectIdError;

            #[inline]
            fn try_from(val: $ty) -> Result<Self, Self::Error> {
                match u64::try_from(val) {
                    Ok(id) => Ok(Self::new(id)),
                    Err(_) => Err(ParseProjectIdError::InvalidValue),
                }
            }
        }
    };
}

impl_try_from!(usize);
impl_try_from!(i8);
impl_try_from!(i16);
impl_try_from!(i32);
impl_try_from!(i64);

impl FromStr for ProjectId {
    type Err = ParseProjectIdError;

    fn from_str(s: &str) -> Result<ProjectId, ParseProjectIdError> {
        if s.is_empty() {
            return Err(ParseProjectIdError::EmptyValue);
        }

        match s.parse::<u64>() {
            Ok(val) => Ok(ProjectId::new(val)),
            Err(_) => Err(ParseProjectIdError::InvalidValue),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_api() {
        let id: ProjectId = "42".parse().unwrap();
        assert_eq!(id, ProjectId::new(42));
        assert_eq!(
            "42xxx".parse::<ProjectId>(),
            Err(ParseProjectIdError::InvalidValue)
        );
        assert_eq!(
            "".parse::<ProjectId>(),
            Err(ParseProjectIdError::EmptyValue)
        );
        assert_eq!(ProjectId::new(42).to_string(), "42");

        assert_eq!(serde_json::to_string(&ProjectId::new(42)).unwrap(), "42");
        assert_eq!(
            serde_json::from_str::<ProjectId>("42").unwrap(),
            ProjectId::new(42)
        );
    }
}
