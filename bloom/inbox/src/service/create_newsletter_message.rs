use super::CreateNewsletterMessageInput;
use crate::{
    entities::{NewsletterMessage, NewsletterMessageStatus},
    Service,
};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn create_newsletter_message(
        &self,
        actor: Actor,
        input: CreateNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        let (namespace, _) = self
            .kernel_service
            .find_namespace_and_membership(&self.db, actor.id, list.namespace_id)
            .await?;

        // clean and validate input
        let name = input.name.trim().to_string();
        self.validate_newsletter_message_name(&name)?;

        let subject = input.subject.trim().to_string();
        self.validate_newsletter_message_subject(&subject)?;

        let body = input.body.trim().to_string();
        self.validate_newsletter_message_body(&body)?;

        let body_html = self.kernel_service.render_markdown(&body).await?;

        let scheduled_for = input.scheduled_for;
        self.validate_newsletter_message_scheduled_for(scheduled_for)?;

        let status = NewsletterMessageStatus::Saved;

        let now = Utc::now();
        let message = NewsletterMessage {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            name,
            subject,
            body,
            body_html,
            scheduled_for,
            status,
            last_sent_at: None,
            sent_count: 0,
            error_count: 0,
            list_id: input.list_id,
            namespace_id: namespace.id,
        };
        self.repo.create_newsletter_message(&self.db, &message).await?;

        Ok(message)
    }
}
