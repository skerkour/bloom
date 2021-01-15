use super::FindNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_message(
        &self,
        actor: Actor,
        input: FindNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, message.namespace_id)
            .await
            .map_err(|_| Error::NewsletterMessageNotFound)?;

        Ok(message)
    }
}
