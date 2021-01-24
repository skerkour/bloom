use stdx::sqlx;

pub enum Error {
    // Files
    FileNotFound,
    FileAlreadyExists,
    FolderIsInTrash,
    FileNameIsTooShort,
    FileNameIsTooLong,
    FileNameIsNotValid,
    StorageLimitReached,
    FileTypeIsNotValid,

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
            // Files
            Error::FileNotFound => kernel::Error::NotFound(String::from("File not found.")),
            Error::FileAlreadyExists => kernel::Error::AlreadyExists(String::from("File already exists.")),
            Error::FolderIsInTrash => kernel::Error::InvalidArgument(String::from("Folder is in trash.")),
            Error::FileNameIsTooShort => kernel::Error::InvalidArgument(String::from("File name is too short.")),
            Error::FileNameIsTooLong => kernel::Error::InvalidArgument(String::from("File name is too long.")),
            Error::FileNameIsNotValid => kernel::Error::InvalidArgument(String::from("File name is not valid.")),
            Error::StorageLimitReached => kernel::Error::PermissionDenied(String::from(
                "Storage limit reached. Please Upgrade your plan to upload more files.",
            )),
            Error::FileTypeIsNotValid => kernel::Error::InvalidArgument(String::from("File type is not valid.")),

            // Other
            Error::Internal => kernel::Error::Internal(String::new()),
            Error::PermissionDenied => kernel::Error::PermissionDenied(String::from("Permission denied.")),
        }
    }
}
