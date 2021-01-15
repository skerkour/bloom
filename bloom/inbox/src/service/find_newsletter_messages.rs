use super::FindNewsletterMessagesInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_messages(
        &self,
        actor: Actor,
        input: FindNewsletterMessagesInput,
    ) -> Result<Vec<NewsletterMessage>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, input.namespace_id)
            .await?;

        let messages = self
            .repo
            .find_newsletter_messages_for_namespace(&self.db, input.namespace_id)
            .await?;

        Ok(messages)
    }
}
