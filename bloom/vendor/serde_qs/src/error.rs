use serde::de;

use std::fmt::Display;
use std::io;
use std::num;
use std::str;
use std::string;

/// Error type for `serde_qs`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Custom string-based error
    #[error("failed with reason: {0}")]
    Custom(String),

    /// Parse error at a specified position in the query string
    #[error("parsing failed with error: '{0}' at position: {1}")]
    Parse(String, usize),

    /// Unsupported type that `serde_qs` can't serialize into a query string
    #[error("unsupported type for serialization")]
    Unsupported,

    /// Error proessing UTF-8 for a `String`
    #[error(transparent)]
    FromUtf8(#[from] string::FromUtf8Error),

    /// I/O error
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Error parsing a number
    #[error(transparent)]
    ParseInt(#[from] num::ParseIntError),

    /// Error processing UTF-8 for a `str`
    #[error(transparent)]
    Utf8(#[from] str::Utf8Error),
}

impl Error {
    /// Generate error to show top-level type cannot be deserialized.
    pub fn top_level(object: &'static str) -> Self {
        Error::Custom(format!(
            "cannot deserialize {} at the top level.\
             Try deserializing into a struct.",
            object
        ))
    }

    /// Generate a parsing error message with position.
    pub fn parse_err<T>(msg: T, position: usize) -> Self
    where
        T: Display,
    {
        Error::Parse(msg.to_string(), position)
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
