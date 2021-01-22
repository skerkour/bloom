use super::CreateNewsletterListInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn create_newsletter_list(
        &self,
        actor: Actor,
        input: CreateNewsletterListInput,
    ) -> Result<NewsletterList, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        // clean and validate input
        let name = input.name.trim().to_string();
        self.validate_newsletter_list_name(&name)?;

        let description = input.description.trim().to_string();
        self.validate_newsletter_list_description(&description)?;

        let now = Utc::now();
        let list = NewsletterList {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,

            name,
            description,

            namespace_id: input.namespace_id,
        };
        self.repo.create_newsletter_list(&self.db, &list).await?;

        Ok(list)
    }
}
