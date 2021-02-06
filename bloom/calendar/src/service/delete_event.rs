use super::DeleteEventInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_event(&self, actor: Actor, input: DeleteEventInput) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let event = self.repo.find_event_by_id(&self.db, input.event_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, event.namespace_id)
            .await?;

        self.repo.delete_event(&self.db, event.id).await?;

        Ok(())
    }
}
