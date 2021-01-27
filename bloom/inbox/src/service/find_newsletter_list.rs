use super::FindNewsletterListInput;
use crate::{service::NewsletterListWithContacts, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_list(
        &self,
        actor: Actor,
        input: FindNewsletterListInput,
    ) -> Result<NewsletterListWithContacts, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, list.namespace_id)
            .await
            .map_err(|_| Error::NewsletterListNotFound)?;

        let contacts = self.repo.find_contacts_for_list(&self.db, list.id).await?;

        Ok(NewsletterListWithContacts {
            list,
            contacts,
        })
    }
}
