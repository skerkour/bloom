use stdx::{
    chrono::{DateTime, Utc},
    encoding::hex,
    url::Url,
};

use crate::{consts, Error, Service};

impl Service {
    pub fn validate_contact_name(&self, name: &str) -> Result<(), Error> {
        if name.len() > consts::CONTACT_NAME_MAX_LENGTH {
            return Err(Error::ContactNameIsTooLong);
        }

        if name.contains('\n') {
            return Err(Error::ContactNameIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_birthday(&self, birthday: Option<DateTime<Utc>>) -> Result<(), Error> {
        if birthday.is_none() {
            return Ok(());
        }

        let birthday = birthday.unwrap();
        let now = Utc::now();

        if birthday > now {
            return Err(Error::ContactBirthdayCantBeInTheFuture);
        }

        Ok(())
    }

    pub fn validate_contact_phone(&self, phone: &str) -> Result<(), Error> {
        // TODO
        if phone.len() > consts::CONTACT_PHONE_MAX_LENGTH {
            return Err(Error::ContactPhoneIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_pgp_key(&self, pgp_key: &str) -> Result<(), Error> {
        // TODO
        if pgp_key.len() > consts::CONTACT_PGP_KEY_MAX_LENGTH {
            return Err(Error::ContactPgpKeyIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_website(&self, website: &str) -> Result<(), Error> {
        if website.is_empty() {
            return Ok(());
        }

        if website.len() > consts::CONTACT_WEBSITE_MAX_LENGTH {
            return Err(Error::ContactWebsiteIsNotValid);
        }

        let url = Url::parse(website).map_err(|_| Error::ContactWebsiteIsNotValid)?;

        let scheme = url.scheme();
        let host = url.host().ok_or(Error::ContactWebsiteIsNotValid)?;
        if (scheme != "http" && scheme != "https") || host.to_string().is_empty() {
            return Err(Error::ContactWebsiteIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_twitter(&self, twitter: &str) -> Result<(), Error> {
        // TODO
        if twitter.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactTwitterIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_instagram(&self, instagram: &str) -> Result<(), Error> {
        // TODO
        if instagram.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactInstagramIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_facebook(&self, facebook: &str) -> Result<(), Error> {
        // TODO
        if facebook.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactFacebookIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_bloom(&self, bloom: &str) -> Result<(), Error> {
        // TODO
        if bloom.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactBloomIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_linkedin(&self, linkedin: &str) -> Result<(), Error> {
        // TODO
        if linkedin.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactLinkedinIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_skype(&self, skype: &str) -> Result<(), Error> {
        // TODO
        if skype.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactSkypeIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_telegram(&self, telegram: &str) -> Result<(), Error> {
        // TODO
        if telegram.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactTelegramIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_notes(&self, notes: &str) -> Result<(), Error> {
        if notes.len() > consts::CONTACT_NOTES_MAX_LENGTH {
            return Err(Error::ContactNotesAreTooLong);
        }

        Ok(())
    }

    pub fn validate_contact_address(&self, address: &str) -> Result<(), Error> {
        if address.len() > consts::CONTACT_ADDRESS_MAX_LENGTH {
            return Err(Error::ContactAddressIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_plan(&self, plan: &str) -> Result<(), Error> {
        // TODO
        if plan.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactPlanIsNotValid);
        }

        Ok(())
    }

    pub fn validate_contact_user_id(&self, user_id: &str) -> Result<(), Error> {
        // TODO
        if user_id.len() > consts::CONTACT_MISC_MAX_LENGTH {
            return Err(Error::ContactUserIdIsNotValid);
        }

        Ok(())
    }

    pub fn validate_newsletter_list_name(&self, name: &str) -> Result<(), Error> {
        if name.len() < consts::LIST_NAME_MIN_LENGTH {
            return Err(Error::NewsletterListNameIsTooShort);
        }

        if name.len() > consts::LIST_NAME_MAX_LENGTH {
            return Err(Error::NewsletterListNameIsTooLong);
        }

        if name.contains('\n') {
            return Err(Error::NewsletterListNameIsNotValid);
        }

        Ok(())
    }

    pub fn validate_newsletter_list_description(&self, description: &str) -> Result<(), Error> {
        if description.len() > consts::LIST_DESCRIPTION_MAX_LENGTH {
            return Err(Error::NewsletterListDescriptionIsTooLong);
        }

        Ok(())
    }

    pub fn validate_newsletter_message_name(&self, name: &str) -> Result<(), Error> {
        if name.len() < consts::NEWSLETTER_MESSAGE_NAME_MIN_LENGTH {
            return Err(Error::NewsletterMessageNameIsTooShort);
        }

        if name.len() > consts::NEWSLETTER_MESSAGE_NAME_MAX_LENGTH {
            return Err(Error::NewsletterMessageNameIsTooLong);
        }

        if name.contains('\n') {
            return Err(Error::NewsletterMessageNameIsNotValid);
        }

        Ok(())
    }

    pub fn validate_newsletter_message_subject(&self, subject: &str) -> Result<(), Error> {
        if subject.len() < consts::NEWSLETTER_MESSAGE_SUBJECT_MIN_LENGTH {
            return Err(Error::NewsletterMessageSubjectIsTooShort);
        }

        if subject.len() > consts::NEWSLETTER_MESSAGE_SUBJECT_MAX_LENGTH {
            return Err(Error::NewsletterMessageSubjectIsTooLong);
        }

        if subject.contains('\n') {
            return Err(Error::NewsletterMessageSubjectIsNotValid);
        }

        Ok(())
    }

    pub fn validate_newsletter_message_body(&self, body: &str) -> Result<(), Error> {
        if body.len() > consts::NEWSLETTER_MESSAGE_BODY_MAX_LENGTH {
            return Err(Error::NewsletterMessageBodyIsTooLong);
        }

        Ok(())
    }

    pub fn validate_newsletter_message_scheduled_for(&self, scheduled_for: Option<DateTime<Utc>>) -> Result<(), Error> {
        if scheduled_for.is_none() {
            return Ok(());
        }

        let scheduled_for = scheduled_for.unwrap();
        let now = Utc::now();
        if scheduled_for < now {
            return Err(Error::NewsletterMessageScheduledForCantBeInThePast);
        }

        Ok(())
    }

    pub fn validate_chatbox_name(&self, name: &str) -> Result<(), Error> {
        if name.len() > consts::CHATBOX_NAME_MAX_LENGTH {
            return Err(Error::ChatboxNameIsTooLong);
        }

        if name.len() < consts::CHATBOX_NAME_MIN_LENGTH {
            return Err(Error::ChatboxNameIsTooShort);
        }

        if !name
            .chars()
            .all(|c| c.is_alphabetic() || c.is_numeric() || "-_. '".contains(c))
        {
            return Err(Error::ChatboxNameIsNotValid);
        }

        Ok(())
    }

    pub fn validate_chatbox_color(&self, color: &str) -> Result<(), Error> {
        if color.len() != 7 {
            return Err(Error::ChatboxColorIsNotValid);
        }

        if color.as_bytes()[0] != b'#' {
            return Err(Error::ChatboxColorIsNotValid);
        }

        hex::decode(&color[1..]).map_err(|_| Error::ChatboxColorIsNotValid)?;

        Ok(())
    }

    pub fn validate_chatbox_welcome_message(&self, welcome_message: &str) -> Result<(), Error> {
        if welcome_message.len() > consts::CHATBOX_WELCOME_MESSAGE_MAX_LENGTH {
            return Err(Error::ChatboxWelcomeMessageIsTooLong);
        }

        Ok(())
    }

    pub fn validate_message_body(&self, body: &str) -> Result<(), Error> {
        todo!();
    }
}
