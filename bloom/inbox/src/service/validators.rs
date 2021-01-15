use stdx::chrono::{DateTime, Utc};

use crate::{Error, Service};

impl Service {
    pub fn validate_conatct_name(&self, name: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_conatct_birthday(&self, birthday: Option<DateTime<Utc>>) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_phone(&self, phone: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_pgp_key(&self, pgp_key: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_website(&self, website: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_twitter(&self, twitter: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_instagram(&self, instagram: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_facebook(&self, facebook: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_bloom(&self, bloom: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_linkedin(&self, linkedin: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_skype(&self, skype: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_telegram(&self, telegram: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_notes(&self, notes: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_address(&self, address: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_plan(&self, plan: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_contact_user_id(&self, user_id: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_newsletter_list_name(&self, name: &str) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_newsletter_list_description(&self, description: &str) -> Result<(), Error> {
        todo!();
    }
}
