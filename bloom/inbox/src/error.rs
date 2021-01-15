use stdx::sqlx;

pub enum Error {
    // Contacts
    ContactNotFound,
    ContactsCsvTooLarge,

    // Chatbox
    UpgradePlanToRemoveChatboxBranding,
    ChatboxPreferencesNotFound,

    // Newsletter
    NewsletterListNotFound,
    NewsletterMessageNotFound,

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
            Error::ContactsCsvTooLarge => kernel::Error::InvalidArgument(String::from("contacts CSV too large.")),

            // Chatbox
            Error::UpgradePlanToRemoveChatboxBranding => {
                kernel::Error::InvalidArgument(String::from("Please upgrade your plan to remove chatbox's branding"))
            }
            Error::ChatboxPreferencesNotFound => kernel::Error::NotFound(String::from("Chatbox not found.")),

            // Newsletter
            Error::NewsletterListNotFound => kernel::Error::NotFound(String::from("List not found")),
            Error::NewsletterMessageNotFound => kernel::Error::NotFound(String::from("Message not found")),

            // Other
            Error::Internal => kernel::Error::Internal,
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
