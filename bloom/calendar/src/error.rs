use stdx::sqlx;

pub enum Error {
    // Events
    EventNotFound,
    EventTitleIsTooLong,
    EventTitleIsInvalid,
    EventDescriptionIsTooLong,
    EventEndDateCantBeBeforeStartDate,
    EventDatesAreInvalid,
    EventLocationIsTooLong,
    EventLocationIsInvalid,

    // Other
    Internal,
    PermissionDenied,
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        // Not found errors should be catched manually
        Error::Internal
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
            Error::EventLocationIsTooLong => kernel::Error::InvalidArgument(String::from("Location is too long.")),
            Error::EventLocationIsInvalid => kernel::Error::InvalidArgument(String::from("Location is not valid.")),

            // Other
            Error::Internal => kernel::Error::Internal(String::new()),
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
