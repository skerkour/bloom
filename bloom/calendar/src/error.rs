use stdx::sqlx;

pub enum Error {
    // Events
    EventNotFound,
    EventTitleIsTooLong,
    EventTitleIsInvalid,
    EventDescriptionIsTooLong,
    EventEndDateCantBeBeforeStartDate,
    EventDatesAreInvalid,

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
            // Events
            Error::EventNotFound => kernel::Error::NotFound(String::from("Event not found.")),
            Error::EventTitleIsTooLong => kernel::Error::InvalidArgument(String::from("Title is too long.")),
            Error::EventTitleIsInvalid => kernel::Error::InvalidArgument(String::from("Title is not valid.")),
            Error::EventDescriptionIsTooLong => {
                kernel::Error::InvalidArgument(String::from("Description is too long."))
            }
            Error::EventDatesAreInvalid => kernel::Error::InvalidArgument(String::from("Dates are not valid.")),
            Error::EventEndDateCantBeBeforeStartDate => {
                kernel::Error::InvalidArgument(String::from("Start date canoot be after End date."))
            }

            // Other
            Error::Internal => kernel::Error::Internal(String::new()),
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
