use super::SubscribeToListInput;
use crate::{
    entities::{Contact, NewsletterListSubscription},
    Error, Service,
};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid, uuid::Uuid};

impl Service {
    // authentication not required
    pub async fn subscribe_to_list(&self, _actor: Actor, input: SubscribeToListInput) -> Result<(), kernel::Error> {
        let name = input.name.unwrap_or(String::new()).trim().to_string();
        let email = input.email.trim().to_lowercase();
        let now = Utc::now();

        self.validate_contact_name(&name)?;
        self.kernel_service.validate_email(&email, false)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        let res = self
            .repo
            .find_contact_by_email(&self.db, list.namespace_id, &email)
            .await;

        let mut tx = self.db.begin().await?;

        let contact = match res {
            Ok(existing_contact) => Ok(existing_contact),
            Err(Error::ContactNotFound) => {
                // create contact
                let new_contact = Contact {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    name: name,
                    birthday: None,
                    email: email,
                    pgp_key: String::new(),
                    phone: String::new(),
                    address: String::new(),
                    website: String::new(),
                    twitter: String::new(),
                    instagram: String::new(),
                    facebook: String::new(),
                    linkedin: String::new(),
                    skype: String::new(),
                    telegram: String::new(),
                    bloom: String::new(),
                    notes: String::new(),
                    plan: String::new(),
                    user_id: String::new(),
                    country: String::new(),
                    country_code: String::new(),
                    namespace_id: list.namespace_id,
                    avatar_id: None,
                };
                self.repo.create_contact(&mut tx, &new_contact).await?;

                Ok(new_contact)
            }
            Err(err) => Err(err),
        }?;

        let subscription_res = self
            .repo
            .find_newsletter_subscription_by_contact_id_and_list_id(&mut tx, contact.id, list.id)
            .await;
        match subscription_res {
            Ok(_) => Ok(()),
            Err(Error::NewsletterSubscriptionNotFound) => {
                // we generate a random Uuid instad of a Ulid to prevent unsubscribe bruteforcing
                let subscription = NewsletterListSubscription {
                    id: Uuid::new_v4(),
                    created_at: now,
                    updated_at: now,
                    list_id: list.id,
                    contact_id: contact.id,
                };
                self.repo
                    .create_newsletter_list_subscription(&mut tx, &subscription)
                    .await?;
                Ok(())
            }
            Err(err) => Err(err),
        }?;

        tx.commit().await?;

        Ok(())
    }
}
