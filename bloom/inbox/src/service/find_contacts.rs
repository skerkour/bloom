use super::FindContactsInput;
use crate::{entities::Contact, Service};
use kernel::Actor;

impl Service {
    pub async fn find_contacts(&self, actor: Actor, input: FindContactsInput) -> Result<Vec<Contact>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let contacts = self
            .repo
            .find_contacts_for_namespace(&self.db, input.namespace_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, input.namespace_id)
            .await?;

        Ok(contacts)
    }
}
