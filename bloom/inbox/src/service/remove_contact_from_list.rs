use super::RemoveContactFromListInput;
use crate::{Error, Service};
use kernel::Actor;

impl Service {
    pub async fn remove_contact_from_list(
        &self,
        actor: Actor,
        input: RemoveContactFromListInput,
    ) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, list.namespace_id)
            .await
            .map_err(|_| Error::NewsletterListNotFound)?;

        let contact = self.repo.find_contact_by_id(&self.db, input.contact_id).await?;

        if contact.namespace_id != list.namespace_id {
            return Err(Error::PermissionDenied.into());
        }

        let subscription = self
            .repo
            .find_newsletter_subscription_by_contact_id_and_list_id(&self.db, contact.id, list.id)
            .await?;

        self.repo
            .delete_newsletter_list_subscription(&self.db, subscription.id)
            .await?;

        Ok(())
    }
}
