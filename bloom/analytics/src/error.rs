use stdx::sqlx;

pub enum Error {
    // Events
    EventNotValid,

    // Other
    Internal,
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            // Not found errors should be catched manually
            _ => Error::Internal,
        }
    }
}
