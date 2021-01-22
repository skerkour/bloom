use super::FindNewsletterListsInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_lists(
        &self,
        actor: Actor,
        input: FindNewsletterListsInput,
    ) -> Result<Vec<NewsletterList>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let lists = self
            .repo
            .find_newsletter_lists_for_namespace(&self.db, input.namespace_id)
            .await?;

        Ok(lists)
    }
}
