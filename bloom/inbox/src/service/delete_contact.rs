use super::DeleteContactInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_contact(&self, actor: Actor, input: DeleteContactInput) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let contact = self.repo.find_contact_by_id(&self.db, input.contact_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, contact.namespace_id)
            .await?;

        self.repo.delete_contact(&self.db, contact.id).await?;

        Ok(())
    }
}
