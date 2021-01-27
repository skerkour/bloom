use super::{FindNewsletterMessageInput, NewsletterMessageWithLists};
use crate::{Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_message(
        &self,
        actor: Actor,
        input: FindNewsletterMessageInput,
    ) -> Result<NewsletterMessageWithLists, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, message.namespace_id)
            .await
            .map_err(|_| Error::NewsletterMessageNotFound)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, message.list_id).await?;
        let lists = self
            .repo
            .find_newsletter_lists_for_namespace(&self.db, message.namespace_id)
            .await?;

        Ok(NewsletterMessageWithLists {
            message,
            list,
            lists,
        })
    }
}
