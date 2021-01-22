use super::UpdateNewsletterListInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;
use stdx::chrono::Utc;

impl Service {
    pub async fn update_newsletter_list(
        &self,
        actor: Actor,
        input: UpdateNewsletterListInput,
    ) -> Result<NewsletterList, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let mut list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, list.namespace_id)
            .await?;

        let name = input.name.trim().to_string();
        self.validate_newsletter_list_name(&name)?;

        let description = input.description.trim().to_string();
        self.validate_newsletter_list_description(&description)?;

        list.updated_at = Utc::now();
        list.name = name;
        list.description = description;
        self.repo.update_newsletter_list(&self.db, &list).await?;

        Ok(list)
    }
}
