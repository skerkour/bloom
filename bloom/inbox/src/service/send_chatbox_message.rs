use super::SendChatboxMessageInput;
use crate::{
    consts,
    entities::{Conversation, Message},
    Error, Service,
};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn send_chatbox_message(
        &self,
        actor: Actor,
        input: SendChatboxMessageInput,
    ) -> Result<Message, kernel::Error> {
        let anonymous_id = self.kernel_service.current_anonymous_id(actor)?;
        let now = Utc::now();
        let namespace_id = input.namespace_id;

        let mut conversation = match self
            .repo
            .find_inbox_conversation_for_anonymous_id(&self.db, anonymous_id, namespace_id)
            .await
        {
            Ok(conversation) => Ok(conversation),
            Err(Error::ConversationNotFound) => {
                // create conversation
                let new_conversation = Conversation {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    archived_at: None,
                    trashed_at: None,
                    last_message_at: now,
                    is_spam: false,
                    name: String::from(consts::VISITOR),
                    description: String::new(),
                    namespace_id,
                    anonymous_id: Some(anonymous_id),
                };
                self.repo.create_conversation(&self.db, &new_conversation).await?;
                Ok(new_conversation)
            }
            Err(err) => Err(err),
        }?;

        if conversation.anonymous_id != Some(anonymous_id) {
            return Err(Error::PermissionDenied.into());
        }

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
            author_id: None,
            from_operator: false,
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
