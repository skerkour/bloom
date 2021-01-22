use super::FindContactInput;
use crate::{entities::Contact, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_contact(&self, actor: Actor, input: FindContactInput) -> Result<Contact, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let contact = self.repo.find_contact_by_id(&self.db, input.contact_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, contact.namespace_id)
            .await
            .map_err(|_| Error::ContactNotFound)?;

        Ok(contact)
    }
}
