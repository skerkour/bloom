use stdx::sqlx;

pub enum Error {
    // Contacts
    ContactNotFound,
    ContactsCsvTooLarge,
    ContactNameIsTooLong,
    ContactNameIsNotValid,
    ContactBirthdayCantBeInTheFuture,
    ContactWebsiteIsNotValid,
    ContactNotesAreTooLong,
    ContactPhoneIsNotValid,
    ContactPgpKeyIsNotValid,
    ContactTwitterIsNotValid,
    ContactInstagramIsNotValid,
    ContactFacebookIsNotValid,

    // Chatbox
    UpgradePlanToRemoveChatboxBranding,
    ChatboxPreferencesNotFound,

    // Newsletter
    NewsletterListNotFound,
    NewsletterMessageNotFound,
    NewsletterSubscriptionNotFound,

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
            Error::ContactNameIsTooLong => kernel::Error::InvalidArgument(String::from("Name is too long.")),
            Error::ContactNameIsNotValid => kernel::Error::InvalidArgument(String::from("Name is not valid.")),
            Error::ContactBirthdayCantBeInTheFuture => {
                kernel::Error::InvalidArgument(String::from("Birthday can't be in the future."))
            }
            Error::ContactWebsiteIsNotValid => kernel::Error::InvalidArgument(String::from("Website is not valid.")),
            Error::ContactNotesAreTooLong => kernel::Error::InvalidArgument(String::from("Notes are too long.")),
            Error::ContactPhoneIsNotValid => kernel::Error::InvalidArgument(String::from("Phone is not valid.")),
            Error::ContactPgpKeyIsNotValid => kernel::Error::InvalidArgument(String::from("PGP key is not valid.")),
            Error::ContactTwitterIsNotValid => kernel::Error::InvalidArgument(String::from("Twitter is not valid.")),
            Error::ContactInstagramIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Instagram is not valid."))
            }
            Error::ContactFacebookIsNotValid => kernel::Error::InvalidArgument(String::from("Facebook is not valid.")),

            // Chatbox
            Error::UpgradePlanToRemoveChatboxBranding => {
                kernel::Error::InvalidArgument(String::from("Please upgrade your plan to remove chatbox's branding"))
            }
            Error::ChatboxPreferencesNotFound => kernel::Error::NotFound(String::from("Chatbox not found.")),

            // Newsletter
            Error::NewsletterListNotFound => kernel::Error::NotFound(String::from("List not found")),
            Error::NewsletterMessageNotFound => kernel::Error::NotFound(String::from("Message not found")),
            Error::NewsletterSubscriptionNotFound => kernel::Error::NotFound(String::from("Subscription not found")),

            // Other
            Error::Internal => kernel::Error::Internal,
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
