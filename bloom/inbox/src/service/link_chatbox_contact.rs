use super::LinkChatboxContactInput;
use crate::{
    consts,
    entities::{Contact, ContactAnonymousIdRelation, ConversationContactRelation},
    Error, Service,
};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn link_chatbox_contact(
        &self,
        actor: Actor,
        input: LinkChatboxContactInput,
    ) -> Result<(), kernel::Error> {
        let anonymous_id = self.kernel_service.current_anonymous_id(actor)?;
        let now = Utc::now();
        let namespace_id = input.namespace_id;
        let email = input.email.trim().to_lowercase();

        self.kernel_service.validate_email(&email, false)?;

        let mut conversation = self
            .repo
            .find_inbox_conversation_for_anonymous_id(&self.db, anonymous_id, namespace_id)
            .await?;

        let mut tx = self.db.begin().await?;

        let email_parts: Vec<String> = email.split('@').map(|part| part.to_string()).collect();
        let name = email_parts
            .get(0)
            .map(|name| name.to_owned())
            .unwrap_or(conversation.name.clone());

        let contact = match self
            .repo
            .find_contact_by_anonymous_id(&mut tx, anonymous_id, namespace_id)
            .await
        {
            Ok(mut contact) => {
                if email != contact.email {
                    contact.updated_at = now;
                    contact.email = email;
                    if contact.name == consts::VISITOR && name != consts::VISITOR {
                        contact.name = name.clone();
                    }
                    self.repo.update_contact(&mut tx, &contact).await?;
                }

                Ok(contact)
            }
            Err(Error::ContactNotFound) => {
                // create contact
                let new_contact = Contact {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    name: name.clone(),
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
                    namespace_id,
                    avatar_id: None,
                };
                self.repo.create_contact(&mut tx, &new_contact).await?;

                Ok(new_contact)
            }
            Err(err) => Err(err),
        }?;

        let contact_relation = ContactAnonymousIdRelation {
            contact_id: contact.id,
            anonymous_id,
        };
        self.repo
            .create_contact_anonymous_id_relation(&mut tx, &contact_relation)
            .await?;

        let conversation_relation = ConversationContactRelation {
            contact_id: contact.id,
            conversation_id: conversation.id,
        };
        self.repo
            .create_conversation_contact_relation(&mut tx, &conversation_relation)
            .await?;

        conversation.updated_at = now;
        conversation.name = name;
        self.repo.update_conversation(&mut tx, &conversation).await?;

        tx.commit().await?;

        Ok(())
    }
}
