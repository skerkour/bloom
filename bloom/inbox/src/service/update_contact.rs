use super::UpdateContactInput;
use crate::{entities::Contact, Service};
use kernel::Actor;
use stdx::chrono::Utc;

impl Service {
    pub async fn update_contact(&self, actor: Actor, input: UpdateContactInput) -> Result<Contact, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let mut contact = self.repo.find_contact_by_id(&self.db, input.contact_id).await?;

        // check namespace membership
        self.kernel_service
            .check_namespace_membership(&self.db, &actor, contact.namespace_id)
            .await?;

        // clean and validate input
        let email = input.email.trim().to_lowercase();
        if !email.is_empty() {
            self.kernel_service.validate_email(&email, false)?;
        }

        let name = input.name.trim().to_string();
        self.validate_contact_name(&name)?;

        let birthday = input.birthday;
        self.validate_contact_birthday(birthday)?;

        let phone = input.phone.trim().to_string();
        self.validate_contact_phone(&phone)?;

        let pgp_key = input.pgp_key.trim().to_string();
        self.validate_contact_pgp_key(&pgp_key)?;

        let website = input.website.trim().to_string();
        self.validate_contact_website(&website)?;

        let twitter = input.twitter.trim().to_string();
        self.validate_contact_twitter(&twitter)?;

        let instagram = input.instagram.trim().to_string();
        self.validate_contact_instagram(&instagram)?;

        let facebook = input.facebook.trim().to_string();
        self.validate_contact_facebook(&facebook)?;

        let bloom = input.bloom.trim().to_string();
        self.validate_contact_bloom(&bloom)?;

        let linkedin = input.linkedin.trim().to_string();
        self.validate_contact_linkedin(&linkedin)?;

        let skype = input.skype.trim().to_string();
        self.validate_contact_skype(&skype)?;

        let telegram = input.telegram.trim().to_string();
        self.validate_contact_telegram(&telegram)?;

        let notes = input.notes.trim().to_string();
        self.validate_contact_notes(&notes)?;

        let address = input.address.trim().to_string();
        self.validate_contact_address(&address)?;

        let plan = input.plan.trim().to_string();
        self.validate_contact_plan(&plan)?;

        let user_id = input.user_id.trim().to_string();
        self.validate_contact_user_id(&user_id)?;

        let now = Utc::now();

        contact.updated_at = now;
        contact.name = name;
        contact.birthday = birthday;
        contact.email = email;
        contact.pgp_key = pgp_key;
        contact.phone = phone;
        contact.address = address;
        contact.website = website;
        contact.twitter = twitter;
        contact.instagram = instagram;
        contact.facebook = facebook;
        contact.linkedin = linkedin;
        contact.skype = skype;
        contact.telegram = telegram;
        contact.bloom = bloom;
        contact.notes = notes;
        contact.plan = plan;
        contact.user_id = user_id;
        self.repo.update_contact(&self.db, &contact).await?;

        Ok(contact)
    }
}
