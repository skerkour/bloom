use std::{error, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Unknown,
    ContextBadSize,
    KeyTooShort,
    KeyTooLong,
    TagTooShort,
    DigestTooShort,
    DigestTooLong,
    InitializationFailed,
    StreamingAeadTagIsNotValid,
    StreamAlreadyFinalized,
    StreamHeaderInvalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
