use stdx::sqlx;

pub enum Error {
    // Events
    EventNotValid,

    // Visitors
    VisitorNotFound,

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

impl std::convert::From<Error> for kernel::Error {
    fn from(err: Error) -> Self {
        match err {
            // Events
            Error::EventNotValid => kernel::Error::InvalidArgument(String::from("event is not valid.")),

            // Visitor
            Error::VisitorNotFound => kernel::Error::NotFound(String::from("Visitor not found.")),

            // Other
            Error::Internal => kernel::Error::Internal(String::new()),
        }
    }
}
