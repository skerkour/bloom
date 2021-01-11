use crate::{consts, Error, Service};

impl Service {
    pub fn validate_file_name(&self, name: &str) -> Result<(), Error> {
        if name.len() < consts::MIN_FILENAME_LENGTH {
            return Err(Error::FileNameIsTooShort);
        }

        if name.len() > consts::MAX_FILENAME_LENGTH {
            return Err(Error::FileNameIsTooLong);
        }

        if name.trim() != name {
            return Err(Error::FileNameIsNotValid);
        }

        if name.to_uppercase() == consts::ROOT_FILE_NAME {
            return Err(Error::FileNameIsNotValid);
        }

        if name.contains('\n') {
            return Err(Error::FileNameIsNotValid);
        }

        Ok(())
    }

    pub fn validate_file_type(&self, filetype: &str) -> Result<(), Error> {
        let lower = filetype.to_lowercase();

        if lower != filetype {
            return Err(Error::FileTypeIsNotValid);
        }

        if filetype.len() < consts::MIME_TYPE_MIN_LENGTH {
            return Err(Error::FileTypeIsNotValid);
        }

        if filetype.len() > consts::MIME_TYPE_MAX_LENGTH {
            return Err(Error::FileTypeIsNotValid);
        }

        if filetype.matches('/').count() != 1 {
            return Err(Error::FileTypeIsNotValid);
        }

        if filetype == consts::FILE_TYPE_FOLDER || filetype.contains("application/com.bloom") {
            return Err(Error::FileTypeIsNotValid);
        }

        Ok(())
    }
}
