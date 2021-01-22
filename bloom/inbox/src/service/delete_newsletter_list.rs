use super::DeleteNewsletterListInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_newsletter_list(
        &self,
        actor: Actor,
        input: DeleteNewsletterListInput,
    ) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, list.namespace_id)
            .await?;

        self.repo.delete_newsletter_list(&self.db, list.id).await?;

        Ok(())
    }
}
