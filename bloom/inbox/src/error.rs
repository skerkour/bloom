use stdx::sqlx;

pub enum Error {
    // Contacts
    ContactNotFound,

    // Other
    Internal,
    PermissionDenied,
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
            // Contacts
            Error::ContactNotFound => kernel::Error::NotFound(String::from("Contact not found.")),

            // Other
            Error::Internal => kernel::Error::Internal,
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
