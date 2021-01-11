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

    pub fn validate_file_type(&self, fileytpe: &str) -> Result<(), Error> {
        todo!();
    }
}
