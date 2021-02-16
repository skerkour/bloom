use super::Service;
use crate::{consts, errors::kernel::Error};
use stdx::validator;

impl Service {
    pub fn validate_email(&self, email: &str, reject_disposable: bool) -> Result<(), Error> {
        if email.is_empty() {
            return Err(Error::EmailIsInvalid);
        }

        if !validator::email(email) {
            return Err(Error::EmailIsInvalid);
        }

        if email.to_lowercase() != email {
            return Err(Error::EmailIsInvalid);
        }

        let parts: Vec<String> = email.split('@').map(|part| part.to_string()).collect();
        if parts.len() != 2 {
            return Err(Error::EmailIsInvalid);
        }

        if parts[1].matches('.').count() < 1 {
            return Err(Error::EmailIsInvalid);
        }

        if reject_disposable {
            if self.config.mail.domains_blocklist.contains(&parts[1]) {
                return Err(Error::EmailIsInvalid);
            }
        }

        Ok(())
    }

    pub fn validate_username(&self, username: &str) -> Result<(), Error> {
        match self.validate_namespace(username) {
            Ok(_) => Ok(()),
            Err(Error::NamespaceIsTooLong) => Err(Error::UsernameIsTooLong),
            Err(Error::NamespaceIsTooShort) => Err(Error::UsernameIsTooShort),
            Err(Error::InvalidNamespace) => Err(Error::UsernameIsInvalid),
            Err(err) => Err(err),
        }
    }

    pub fn validate_user_description(&self, description: &str) -> Result<(), Error> {
        if description.len() > consts::USER_DESCRIPTION_MAX_LENGTH {
            return Err(Error::UserDescriptionIsTooLong);
        }

        Ok(())
    }

    pub fn validate_user_name(&self, name: &str) -> Result<(), Error> {
        let name_len = name.len();

        if name_len > consts::USER_NAME_MAX_LENGTH {
            return Err(Error::UserNameIsTooLong);
        }

        if name_len < consts::USER_NAME_MIN_LENGTH {
            return Err(Error::UserNameIsTooShort);
        }

        if !name
            .chars()
            .all(|c| c.is_alphabetic() || c.is_numeric() || "-_. '".contains(c))
        {
            return Err(Error::InvalidUserName);
        }

        if name.contains("--") || name.contains("__") || name.contains("..") || name.contains("''") {
            return Err(Error::InvalidUserName);
        }

        Ok(())
    }

    pub fn validate_namespace(&self, namespace: &str) -> Result<(), Error> {
        let namespace_len = namespace.len();

        if namespace_len > consts::NAMESPACE_MAX_LENGTH {
            return Err(Error::NamespaceIsTooLong);
        }

        if namespace_len < consts::NAMESPACE_MIN_LENGTH {
            return Err(Error::NamespaceIsTooShort);
        }

        if self.invalid_namespaces.contains(namespace) {
            return Err(Error::InvalidNamespace);
        }

        if !namespace.chars().all(|c| self.valid_namespace_alphabet.contains(&c)) {
            return Err(Error::InvalidNamespace);
        }

        if namespace.contains("--") || namespace.starts_with("-") || namespace.ends_with("-") {
            return Err(Error::InvalidNamespace);
        }

        Ok(())
    }

    pub fn validate_group_name(&self, name: &str) -> Result<(), Error> {
        let name_len = name.len();

        if name_len > consts::GROUP_NAME_MAX_LENGTH {
            return Err(Error::GroupNameIsTooLong);
        }

        if name_len < consts::GROUP_NAME_MIN_LENGTH {
            return Err(Error::GroupNameIsTooShort);
        }

        if !name
            .chars()
            .all(|c| c.is_alphabetic() || c.is_numeric() || "-_. '".contains(c))
        {
            return Err(Error::InvalidGroupName);
        }

        if name.contains("--") || name.contains("__") || name.contains("..") || name.contains("''") {
            return Err(Error::InvalidGroupName);
        }

        Ok(())
    }

    pub fn validate_group_description(&self, description: &str) -> Result<(), Error> {
        if description.len() > consts::GROUP_DESCRIPTION_MAX_LENGTH {
            return Err(Error::GroupDescriptionIsTooLong);
        }

        Ok(())
    }

    pub fn validate_upload_size(&self, size: u64) -> Result<(), Error> {
        if size > consts::UPLOAD_MAX_SIZE {
            return Err(Error::FileIsTooLarge);
        }

        Ok(())
    }

    pub fn validate_qr_code_input(&self, input: &str) -> Result<(), Error> {
        if input.len() > consts::QR_CODE_INPUT_MAX_LENGTH {
            return Err(Error::FileIsTooLarge);
        }

        Ok(())
    }

    pub fn validate_customer_email(&self, email: &str) -> Result<(), Error> {
        if email.is_empty() {
            return Err(Error::CustomerEmailRequired);
        }

        if email.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH {
            return Err(Error::CustomerEmailIsTooLong);
        }

        Ok(())
    }

    pub fn validate_customer_name(&self, name: &str) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::CustomerNameRequired);
        }

        if name.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH {
            return Err(Error::CustomerNameIsTooLong);
        }

        Ok(())
    }

    pub fn validate_customer_city(&self, city: &str) -> Result<(), Error> {
        if city.is_empty() {
            return Err(Error::CustomerCityRequired);
        }

        if city.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH {
            return Err(Error::CustomerCityIsTooLong);
        }

        Ok(())
    }

    pub fn validate_customer_postal_code(&self, postal_code: &str) -> Result<(), Error> {
        if postal_code.is_empty() {
            return Err(Error::CustomerPostalCodeRequired);
        }

        if postal_code.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH {
            return Err(Error::CustomerPostalCodeIsTooLong);
        }

        Ok(())
    }

    pub fn validate_customer_state(&self, state: &str) -> Result<(), Error> {
        if state.is_empty() {
            return Err(Error::CustomerStateRequired);
        }

        if state.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH {
            return Err(Error::CustomerStateIsTooLong);
        }

        Ok(())
    }

    pub fn validate_customer_address(&self, line1: &str, line2: &str) -> Result<(), Error> {
        if line1.is_empty() {
            return Err(Error::CustomerAddressRequired);
        }

        if line1.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH || line2.len() > consts::CUSTOMER_PROPERTY_MAX_LENGTH {
            return Err(Error::CustomerAddressIsTooLong);
        }

        Ok(())
    }
}
