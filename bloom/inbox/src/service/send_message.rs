use super::SendMessageInput;
use crate::{entities::Message, Error, Service};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn send_message(&self, actor: Actor, input: SendMessageInput) -> Result<Message, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;
        let now = Utc::now();

        let mut conversation = self
            .repo
            .find_conversation_by_id(&self.db, input.conversation_id)
            .await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, conversation.namespace_id)
            .await
            .map_err(|_| Error::ConversationNotFound)?;

        let body = input.body.trim().to_string();
        self.validate_inbox_message_body(&body)?;

        let body_html = self.xss.escape(&body);
        // remove repeated newlines
        let body_html = body_html
            .split("\n")
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .collect::<Vec<&str>>()
            .join(" <br /> \n");

        let message = Message {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            received_at: now,
            body_html,
            conversation_id: conversation.id,
            author_id: Some(actor.id),
            from_operator: true,
        };
        self.repo.create_inbox_message(&self.db, &message).await?;

        // we can ignore error as it's not that important
        conversation.updated_at = now;
        conversation.last_message_at = now;
        conversation.archived_at = None;
        let _ = self.repo.update_conversation(&self.db, &conversation).await;

        Ok(message)
    }
}
