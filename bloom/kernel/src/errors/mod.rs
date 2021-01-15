use stdx::sqlx;
use thiserror::Error;

pub mod kernel;

// pub type Error = Box<dyn std::error::Error>;
// pub type Error = anyhow::Error;
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal,
    #[error("{0}")]
    NotFound(String),
    #[error("Authentication required.")]
    AuthenticationRequired,
    #[error("{0}")]
    PermissionDenied(String),
    #[error("{0}")]
    InvalidArgument(String),
    #[error("{0}")]
    AlreadyExists(String),
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("Not found".into()),
            _ => Error::Internal,
        }
    }
}

impl std::convert::From<actix_web::Error> for Error {
    fn from(_: actix_web::Error) -> Self {
        Error::Internal
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::Internal
    }
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => Error::NotFound("Env var not found".into()),
            _ => Error::Internal,
        }
    }
}

impl std::convert::From<sqlx::migrate::MigrateError> for Error {
    fn from(_err: sqlx::migrate::MigrateError) -> Self {
        Error::Internal
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<stdx::encoding::base64::DecodeError> for Error {
    fn from(err: stdx::encoding::base64::DecodeError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<std::str::ParseBoolError> for Error {
    fn from(err: std::str::ParseBoolError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<stdx::sync::threadpool::Error> for Error {
    fn from(err: stdx::sync::threadpool::Error) -> Self {
        match err {
            _ => Error::Internal,
        }
    }
}

impl std::convert::From<std::string::FromUtf8Error> for Error {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        Error::Internal
    }
}

impl std::convert::From<stdx::image::error::ImageError> for Error {
    fn from(_err: stdx::image::error::ImageError) -> Self {
        Error::Internal
    }
}

impl std::convert::From<stdx::csv::Error> for Error {
    fn from(_err: stdx::csv::Error) -> Self {
        Error::Internal
    }
}
