use super::FindNewsletterListInput;
use crate::{service::NewsletterListWithDetails, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_list(
        &self,
        actor: Actor,
        input: FindNewsletterListInput,
    ) -> Result<NewsletterListWithDetails, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, list.namespace_id)
            .await
            .map_err(|_| Error::NewsletterListNotFound)?;

        let contacts = self.repo.find_contacts_for_list(&self.db, list.id).await?;

        let messages = self.repo.find_newsletter_messages_by_list_id(&self.db, list.id).await?;

        let acquisition = self.repo.find_newsletter_list_acquisition(&self.db, list.id).await?;

        Ok(NewsletterListWithDetails {
            list,
            contacts,
            messages,
            acquisition,
        })
    }
}
