/// The Error type.
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
}

/// The kind of an error. For the time being, it will remain private.
#[derive(Debug)]
pub(crate) enum ErrorKind {
    Utf8(std::str::Utf8Error),
    FromUtf16(::std::string::FromUtf16Error),
    Io(::std::io::Error),
    Os(u32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Utf8(err) => write!(f, "Utf8 error: {}", err),
            ErrorKind::FromUtf16(err) => write!(f, "FromUtf16 error: {}", err),
            ErrorKind::Io(err) => write!(f, "IO error: {}", err),
            ErrorKind::Os(err) => write!(f, "OS error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Utf8(err) => Some(err),
            ErrorKind::FromUtf16(err) => Some(err),
            ErrorKind::Io(err) => Some(err),
            ErrorKind::Os(_) => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;


impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error {kind: ErrorKind::Utf8(err)}
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(err: std::string::FromUtf16Error) -> Self {
        Error {kind: ErrorKind::FromUtf16(err)}
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {kind: ErrorKind::Io(err)}
    }
}
