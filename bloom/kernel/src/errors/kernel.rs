use stdx::sqlx;

#[derive(Debug)]
pub enum Error {
    // Other
    FileSizeIsNegative,
    FileIsTooLarge,
    InvalidAvatarId,
    PermissionDenied,
    SoftLimitReached,
    Internal,
    EmailIsInvalid,
    UploadNotFound,
    QrCodeInputIsTooLong,
    MarkdownIsTooLong,
    CountryNotValid,

    // User
    UserNotFound,
    MustNotBeAuthenticated,
    EmailAlreadyExists,
    UsernameAlreadyExists,
    InvalidRegistrationCode,
    MaxRegistrationAttempsReached,
    RegistrationCodeExpired,
    UserIsBcloked,
    MaxSignInAttempsReached,
    SignInCodeExpired,
    InvalidSignInCode,
    AuthenticationRequired,
    PendingEmailNotFound,
    MaxEmailVerificationAttempsReached,
    EmailVerificationCodeExpired,
    InvalidEmailVerificationCode,
    TwoFACodeIsNotValid,
    AdminUserCantBeDeleted,
    LeaveAllGroupsBeforeDeletingAccount,
    TwoFaIsNotEnabled,
    TwoFaAlreadyEnabled,
    SomeUsersNotFound,
    UserDescriptionIsTooLong,
    UserNameIsTooLong,
    UserNameIsTooShort,
    InvalidUserName,
    CantBlockAdmin,
    UsernameIsInvalid,
    UsernameIsTooLong,
    UsernameIsTooShort,

    // Group
    AdminRoleRequired,
    AtLeatOneAdminMustRemainInGroup,
    MembersLimitReachedForPlan,
    GroupNotFound,
    GroupInvitationNotFound,
    GroupMemberNotFound,
    GroupDescriptionIsTooLong,
    GroupNameIsTooLong,
    GroupNameIsTooShort,
    InvalidGroupName,

    // Session
    SessionNotFound,
    InvalidSession,

    // Namespace
    NamespaceNotFound,
    NamespaceAlreadyExists,
    NamespaceIsTooLong,
    NamespaceIsTooShort,
    InvalidNamespace,

    // Billing
    CustomerNotFound,
    SubscriptionIsActive,
    VatDoesNotMatchCountry,
    TaxIdIsAcceptedOnlyForEu,
    CustomerNameRequired,
    CustomerNameIsTooLong,
    CustomerEmailRequired,
    CustomerEmailIsTooLong,
    CustomerCityRequired,
    CustomerCityIsTooLong,
    CustomerPostalCodeRequired,
    CustomerPostalCodeIsTooLong,
    CustomerStateRequired,
    CustomerStateIsTooLong,
    CustomerAddressRequired,
    CustomerAddressIsTooLong,
    BillingCantBeAccessedWhenSelfHosting,
}

impl std::convert::From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            // Users
            Error::UserNotFound => crate::Error::NotFound(String::from("User not found.")),
            Error::MustNotBeAuthenticated => crate::Error::PermissionDenied(String::from("Must not be authenticated.")),
            Error::EmailAlreadyExists => {
                crate::Error::AlreadyExists(String::from("Email is associated with an existing account."))
            }
            Error::UsernameAlreadyExists => crate::Error::AlreadyExists(String::from("Username is already in use.")),
            Error::InvalidRegistrationCode => {
                crate::Error::InvalidArgument(String::from("Confirmation code is not valid. Please try again."))
            }
            Error::MaxRegistrationAttempsReached => crate::Error::InvalidArgument(String::from(
                "Max confirmation attemps reached. Please create a new account.",
            )),
            Error::RegistrationCodeExpired => {
                crate::Error::InvalidArgument(String::from("Confirmation code expired. Please create a new account."))
            }
            Error::UserIsBcloked => crate::Error::PermissionDenied(String::from(
                "Account deactivated. Please contact support to re-activate your account.",
            )),
            Error::MaxSignInAttempsReached => {
                crate::Error::InvalidArgument(String::from("Max confirmation attemps reached. Please sign in again."))
            }
            Error::SignInCodeExpired => {
                crate::Error::InvalidArgument(String::from("Confirmation code expired. Please sign in again."))
            }
            Error::InvalidSignInCode => {
                crate::Error::InvalidArgument(String::from("Confirmation code is not valid. Please try again."))
            }
            Error::AuthenticationRequired => crate::Error::AuthenticationRequired,
            Error::PendingEmailNotFound => crate::Error::NotFound(String::from("Pending email not found.")),
            Error::MaxEmailVerificationAttempsReached => crate::Error::InvalidArgument(String::from(
                "Max confirmation attemps reached. Please update your email again.",
            )),
            Error::EmailVerificationCodeExpired => crate::Error::InvalidArgument(String::from(
                "Confirmation code expired. Please change your email again.",
            )),
            Error::InvalidEmailVerificationCode => {
                crate::Error::InvalidArgument(String::from("Confirmation code is not valid. Please try again."))
            }
            Error::TwoFACodeIsNotValid => crate::Error::PermissionDenied(String::from("2FA code is not valid.")),
            Error::AdminUserCantBeDeleted => crate::Error::InvalidArgument(String::from("Admin user cant be deleted")),
            Error::LeaveAllGroupsBeforeDeletingAccount => {
                crate::Error::InvalidArgument(String::from("Please leave all your groups to delete your account."))
            }
            Error::TwoFaIsNotEnabled => crate::Error::InvalidArgument(String::from("2FA is not setup.")),
            Error::TwoFaAlreadyEnabled => crate::Error::InvalidArgument(String::from("2FA already enabled.")),
            Error::SomeUsersNotFound => crate::Error::NotFound(String::from("Some users were not found")),
            Error::UserDescriptionIsTooLong => crate::Error::InvalidArgument(String::from("Description is too long.")),
            Error::UserNameIsTooLong => crate::Error::InvalidArgument(String::from("Name is too short.")),
            Error::UserNameIsTooShort => crate::Error::InvalidArgument(String::from("Name is too long.")),
            Error::InvalidUserName => crate::Error::InvalidArgument(String::from("Name is not valid.")),
            Error::CantBlockAdmin => crate::Error::InvalidArgument(String::from("Admins can't be blocked.")),
            Error::UsernameIsInvalid => {
                crate::Error::InvalidArgument(String::from("Username characters are not valid."))
            }
            Error::UsernameIsTooLong => crate::Error::InvalidArgument(String::from("Username is too long.")),
            Error::UsernameIsTooShort => crate::Error::InvalidArgument(String::from("Username is too short.")),
            // Group
            Error::AdminRoleRequired => crate::Error::PermissionDenied(String::from("Administrator role required")),
            Error::AtLeatOneAdminMustRemainInGroup => {
                crate::Error::InvalidArgument(String::from("At leat one administrator must remain in group."))
            }
            Error::MembersLimitReachedForPlan => {
                crate::Error::PermissionDenied(String::from("Please upgrade your plan to invite more people"))
            }
            Error::GroupNotFound => crate::Error::NotFound(String::from("Group not found.")),
            Error::GroupInvitationNotFound => crate::Error::NotFound(String::from("Invitation not found.")),
            Error::GroupMemberNotFound => crate::Error::NotFound(String::from("Group member not found.")),
            Error::GroupDescriptionIsTooLong => {
                crate::Error::InvalidArgument(String::from("Group description is too long."))
            }
            Error::GroupNameIsTooLong => crate::Error::InvalidArgument(String::from("Group name is too short.")),
            Error::GroupNameIsTooShort => crate::Error::InvalidArgument(String::from("Group name is too long.")),
            Error::InvalidGroupName => crate::Error::InvalidArgument(String::from("Group name is not valid.")),

            // Session
            Error::SessionNotFound => crate::Error::NotFound(String::from("Session not found.")),
            Error::InvalidSession => crate::Error::InvalidArgument(String::from("Session is not valid.")),

            // Namespace
            Error::NamespaceNotFound => crate::Error::NotFound(String::from("Namespace not found.")),
            Error::NamespaceAlreadyExists => crate::Error::AlreadyExists(String::from("Namespace already exists.")),
            Error::NamespaceIsTooLong => crate::Error::InvalidArgument(String::from("Namespace is too long.")),
            Error::NamespaceIsTooShort => crate::Error::InvalidArgument(String::from("Namespace is too short.")),
            Error::InvalidNamespace => {
                crate::Error::InvalidArgument(String::from("Namespace characters are not valid."))
            }

            // Billing
            Error::CustomerNotFound => crate::Error::NotFound(String::from("Customer not found.")),
            Error::SubscriptionIsActive => crate::Error::InvalidArgument(String::from(
                "A subscription is active. Please cancel your subscription before deleting your namespace.",
            )),
            Error::VatDoesNotMatchCountry => {
                crate::Error::InvalidArgument(String::from("VAT number does not match country."))
            }
            Error::TaxIdIsAcceptedOnlyForEu => {
                crate::Error::InvalidArgument(String::from("Tax IDs are accepted only for EU companies"))
            }
            Error::CustomerNameRequired => crate::Error::InvalidArgument(String::from("Name is required.")),
            Error::CustomerNameIsTooLong => crate::Error::InvalidArgument(String::from("Name is too long")),
            Error::CustomerEmailRequired => crate::Error::InvalidArgument(String::from("Email is required.")),
            Error::CustomerEmailIsTooLong => crate::Error::InvalidArgument(String::from("Email is too long.")),
            Error::CustomerCityRequired => crate::Error::InvalidArgument(String::from("City is required.")),
            Error::CustomerCityIsTooLong => crate::Error::InvalidArgument(String::from("City is too long.")),
            Error::CustomerPostalCodeRequired => {
                crate::Error::InvalidArgument(String::from("Postal code is required."))
            }
            Error::CustomerPostalCodeIsTooLong => {
                crate::Error::InvalidArgument(String::from("Postal code is too long."))
            }
            Error::CustomerStateRequired => crate::Error::InvalidArgument(String::from("State is required.")),
            Error::CustomerStateIsTooLong => crate::Error::InvalidArgument(String::from("State is too long.")),
            Error::CustomerAddressRequired => crate::Error::InvalidArgument(String::from("Address is required.")),
            Error::CustomerAddressIsTooLong => crate::Error::InvalidArgument(String::from("Address is too long.")),
            Error::BillingCantBeAccessedWhenSelfHosting => {
                crate::Error::PermissionDenied(String::from("Billing can't be accessed when self-hosting."))
            }

            // Other
            Error::FileSizeIsNegative => crate::Error::InvalidArgument("File size can't be negative.".into()),
            Error::FileIsTooLarge => crate::Error::InvalidArgument("File is too large. The limit is 8GB.".into()),
            Error::InvalidAvatarId => crate::Error::InvalidArgument("Avatar ID is not valid.".into()),
            Error::PermissionDenied => crate::Error::PermissionDenied(String::from("Permission denied.")),
            Error::SoftLimitReached => {
                crate::Error::InvalidArgument(String::from("Soft limit reached. Please contact support."))
            }
            Error::Internal => crate::Error::Internal(String::new()),
            Error::EmailIsInvalid => crate::Error::InvalidArgument(String::from("Email is not valid.")),
            Error::UploadNotFound => crate::Error::NotFound(String::from("Upload not found.")),
            Error::QrCodeInputIsTooLong => crate::Error::InvalidArgument(String::from("QR code input is too long.")),
            Error::MarkdownIsTooLong => crate::Error::InvalidArgument(String::from("Markdown is too long.")),
            Error::CountryNotValid => crate::Error::InvalidArgument(String::from("Country is not valid.")),
        }
    }
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            // Not found error should be catched manually
            _ => Error::Internal,
        }
    }
}
