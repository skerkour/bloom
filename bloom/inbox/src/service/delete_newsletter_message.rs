use super::DeleteNewsletterMessageInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_newsletter_message(
        &self,
        actor: Actor,
        input: DeleteNewsletterMessageInput,
    ) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;
        let list = self.repo.find_newsletter_list_by_id(&self.db, message.list_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, list.namespace_id)
            .await?;

        self.repo.delete_newsletter_message(&self.db, message.id).await?;

        Ok(())
    }
}
