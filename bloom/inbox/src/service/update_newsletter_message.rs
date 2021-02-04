use super::UpdateNewsletterMessageInput;
use crate::{service::NewsletterMessageWithLists, Error, Service};
use kernel::Actor;
use stdx::chrono::Utc;

impl Service {
    pub async fn update_newsletter_message(
        &self,
        actor: Actor,
        input: UpdateNewsletterMessageInput,
    ) -> Result<NewsletterMessageWithLists, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let mut message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, message.namespace_id)
            .await?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, input.list_id).await?;

        if message.namespace_id != list.namespace_id {
            return Err(Error::PermissionDenied.into());
        }

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

        message.updated_at = Utc::now();
        message.name = name;
        message.subject = subject;
        message.body = body;
        message.body_html = body_html;
        message.scheduled_for;
        message.list_id = list.id;
        self.repo.update_newsletter_message(&self.db, &message).await?;

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
