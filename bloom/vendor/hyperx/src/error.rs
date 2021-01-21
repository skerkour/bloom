//! Error and Result module.

use std::error::Error as StdError;
use std::fmt;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use httparse;

use self::Error::{
    Method,
    Version,
    Header,
    Status,
    TooLarge,
    Utf8
};

/// Result type often returned from methods that can have hyper `Error`s.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Errors while parsing headers and associated types.
#[derive(Debug)]
pub enum Error {
    /// An invalid `Method`, such as `GE,T`.
    Method,
    /// An invalid `HttpVersion`, such as `HTP/1.1`
    Version,
    /// An invalid `Header`.
    Header,
    /// A message head is too large to be reasonable.
    TooLarge,
    /// An invalid `Status`, such as `1337 ELITE`.
    Status,
    /// Parsing a field as string failed.
    Utf8(Utf8Error),

    #[doc(hidden)]
    __Nonexhaustive(Void)
}

#[doc(hidden)]
pub struct Void(());

impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Utf8(ref e) => fmt::Display::fmt(e, f),
            ref e => f.write_str(e.static_description()),
        }
    }
}

impl Error {
    fn static_description(&self) -> &str {
        match *self {
            Method => "invalid Method specified",
            Version => "invalid HTTP version specified",
            Header => "invalid Header provided",
            TooLarge => "message head is too large",
            Status => "invalid Status provided",
            Utf8(_) => "invalid UTF-8 string",
            Error::__Nonexhaustive(..) => unreachable!(),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.static_description()
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Utf8(ref error) => Some(error),
            Error::__Nonexhaustive(..) => unreachable!(),
            _ => None,
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Utf8(err.utf8_error())
    }
}

impl From<httparse::Error> for Error {
    fn from(err: httparse::Error) -> Error {
        match err {
            httparse::Error::HeaderName |
            httparse::Error::HeaderValue |
            httparse::Error::NewLine |
            httparse::Error::Token => Header,
            httparse::Error::Status => Status,
            httparse::Error::TooManyHeaders => TooLarge,
            httparse::Error::Version => Version,
        }
    }
}

#[doc(hidden)]
trait AssertSendSync: Send + Sync + 'static {}
#[doc(hidden)]
impl AssertSendSync for Error {}

#[cfg(test)]
mod tests {
    use httparse;
    use super::Error;
    use super::Error::*;

    macro_rules! from {
        ($from:expr => $error:pat) => {
            match Error::from($from) {
                e @ $error => {
                    assert!(format!("{}", e).len() >= 5);
                    assert_ne!(
                        format!("{}", e),
                        "description() is deprecated; use Display");
                } ,
                e => panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_from() {
        from!(httparse::Error::HeaderName => Header);
        from!(httparse::Error::HeaderName => Header);
        from!(httparse::Error::HeaderValue => Header);
        from!(httparse::Error::NewLine => Header);
        from!(httparse::Error::Status => Status);
        from!(httparse::Error::Token => Header);
        from!(httparse::Error::TooManyHeaders => TooLarge);
        from!(httparse::Error::Version => Version);
    }
}
