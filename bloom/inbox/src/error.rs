use stdx::sqlx;

#[derive(Debug)]
pub enum Error {
    // Contacts
    ContactNotFound,
    ContactsCsvTooLarge,
    ContactsCsvNotValid,
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
    ContactBloomIsNotValid,
    ContactLinkedinIsNotValid,
    ContactSkypeIsNotValid,
    ContactTelegramIsNotValid,
    ContactAddressIsNotValid,
    ContactPlanIsNotValid,
    ContactUserIdIsNotValid,

    // Conversations
    ConversationNotFound,

    // Messages
    MessageIsEmpty,
    MessageIsTooLong,

    // Chatbox
    UpgradePlanToRemoveChatboxBranding,
    ChatboxPreferencesNotFound,
    ChatboxWelcomeMessageIsTooLong,
    ChatboxColorIsNotValid,
    ChatboxNameIsTooLong,
    ChatboxNameIsTooShort,
    ChatboxNameIsNotValid,
    ChatboxTwitterIsNotValid,
    ChatboxFacebookUrlIsNotValid,
    ChatboxInstagramIsNotValid,
    ChatboxMastodonUrlIsNotValid,
    ChatboxWebsiteUrlIsNotValid,
    ChatboxTelegramIsNotValid,
    ChatboxWhatsAppNumberIsNotValid,

    // Newsletter
    NewsletterListNotFound,
    NewsletterMessageNotFound,
    NewsletterSubscriptionNotFound,
    NewsletterListNameIsTooLong,
    NewsletterListNameIsTooShort,
    NewsletterListNameIsNotValid,
    NewsletterListDescriptionIsTooLong,
    NewsletterMessageNameIsTooLong,
    NewsletterMessageNameIsTooShort,
    NewsletterMessageNameIsNotValid,
    NewsletterMessageBodyIsTooLong,
    NewsletterMessageSubjectIsTooLong,
    NewsletterMessageSubjectIsTooShort,
    NewsletterMessageSubjectIsNotValid,
    NewsletterMessageScheduledForCantBeInThePast,
    UpgradePlanToSendNewsletterMessage,

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
            Error::ContactsCsvNotValid => kernel::Error::InvalidArgument(String::from("contacts CSV not valid.")),
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
            Error::ContactBloomIsNotValid => kernel::Error::InvalidArgument(String::from("Bloom is not valid.")),
            Error::ContactLinkedinIsNotValid => kernel::Error::InvalidArgument(String::from("LinkedIn is not valid.")),
            Error::ContactSkypeIsNotValid => kernel::Error::InvalidArgument(String::from("Skype is not valid.")),
            Error::ContactTelegramIsNotValid => kernel::Error::InvalidArgument(String::from("Telegram is not valid.")),
            Error::ContactAddressIsNotValid => kernel::Error::InvalidArgument(String::from("Address is not valid.")),
            Error::ContactPlanIsNotValid => kernel::Error::InvalidArgument(String::from("Plan is not valid.")),
            Error::ContactUserIdIsNotValid => kernel::Error::InvalidArgument(String::from("User ID is not valid.")),

            // Conversations
            Error::ConversationNotFound => kernel::Error::NotFound(String::from("Conversation not found.")),

            // Messages
            Error::MessageIsEmpty => kernel::Error::NotFound(String::from("Message is empty.")),
            Error::MessageIsTooLong => kernel::Error::NotFound(String::from("Message is too long.")),
            // Chatbox
            Error::UpgradePlanToRemoveChatboxBranding => {
                kernel::Error::InvalidArgument(String::from("Please upgrade your plan to remove chatbox's branding"))
            }
            Error::ChatboxPreferencesNotFound => kernel::Error::NotFound(String::from("Chatbox not found.")),
            Error::ChatboxWelcomeMessageIsTooLong => {
                kernel::Error::InvalidArgument(String::from("Welcome message is too long."))
            }
            Error::ChatboxColorIsNotValid => kernel::Error::InvalidArgument(String::from("Color is not valid.")),
            Error::ChatboxNameIsTooLong => kernel::Error::InvalidArgument(String::from("Name is too long.")),
            Error::ChatboxNameIsTooShort => kernel::Error::InvalidArgument(String::from("Name is too short.")),
            Error::ChatboxNameIsNotValid => kernel::Error::InvalidArgument(String::from("Name is not valid.")),
            Error::ChatboxTwitterIsNotValid => kernel::Error::InvalidArgument(String::from("Twitter is not valid.")),
            Error::ChatboxFacebookUrlIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Facebook URL is not valid."))
            }
            Error::ChatboxInstagramIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Instagram is not valid."))
            }
            Error::ChatboxMastodonUrlIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Mastodon URL is not valid."))
            }
            Error::ChatboxWebsiteUrlIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Website URL is not valid."))
            }
            Error::ChatboxTelegramIsNotValid => kernel::Error::InvalidArgument(String::from("Telegram is not valid.")),
            Error::ChatboxWhatsAppNumberIsNotValid => {
                kernel::Error::InvalidArgument(String::from("WhatsApp number is not valid."))
            }

            // Newsletter
            Error::NewsletterListNotFound => kernel::Error::NotFound(String::from("List not found")),
            Error::NewsletterMessageNotFound => kernel::Error::NotFound(String::from("Message not found")),
            Error::NewsletterSubscriptionNotFound => kernel::Error::NotFound(String::from("Subscription not found")),
            Error::NewsletterListNameIsTooLong => kernel::Error::InvalidArgument(String::from("Name is too long.")),
            Error::NewsletterListNameIsTooShort => kernel::Error::InvalidArgument(String::from("Name is too short.")),
            Error::NewsletterListDescriptionIsTooLong => {
                kernel::Error::InvalidArgument(String::from("Description is too long."))
            }
            Error::NewsletterListNameIsNotValid => kernel::Error::InvalidArgument(String::from("Name is not valid.")),
            Error::NewsletterMessageNameIsTooLong => kernel::Error::InvalidArgument(String::from("Name is too long.")),
            Error::NewsletterMessageNameIsTooShort => {
                kernel::Error::InvalidArgument(String::from("Name is too short."))
            }
            Error::NewsletterMessageNameIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Name is not valid."))
            }
            Error::NewsletterMessageBodyIsTooLong => kernel::Error::InvalidArgument(String::from("Body is too long.")),
            Error::NewsletterMessageSubjectIsTooLong => {
                kernel::Error::InvalidArgument(String::from("Subject is too long."))
            }
            Error::NewsletterMessageSubjectIsTooShort => {
                kernel::Error::InvalidArgument(String::from("Subject is too short."))
            }
            Error::NewsletterMessageSubjectIsNotValid => {
                kernel::Error::InvalidArgument(String::from("Subject is not valid."))
            }
            Error::NewsletterMessageScheduledForCantBeInThePast => {
                kernel::Error::InvalidArgument(String::from("Message can't be scheduled in the past."))
            }
            Error::UpgradePlanToSendNewsletterMessage => {
                kernel::Error::InvalidArgument(String::from("Please upgrade your plan to send newsletter"))
            }

            // Other
            Error::Internal => kernel::Error::Internal(String::new()),
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
