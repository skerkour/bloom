use super::FindChatboxMessagesInput;
use crate::{entities::Message, Error, Service};
use kernel::Actor;

impl Service {
    pub async fn find_chatbox_messages(
        &self,
        actor: Actor,
        input: FindChatboxMessagesInput,
    ) -> Result<Vec<Message>, kernel::Error> {
        let anonymous_id = self.kernel_service.current_anonymous_id(actor)?;

        let conversation_res = self
            .repo
            .find_inbox_conversation_for_anonymous_id(&self.db, anonymous_id, input.namespace_id)
            .await;

        let conversation = match conversation_res {
            Ok(conversation) => Ok(conversation),
            Err(Error::ConversationNotFound) => return Ok(Vec::new()),
            Err(err) => Err(err),
        }?;

        let messages = self
            .repo
            .find_inbox_messages_for_conversation(&self.db, conversation.id, input.after)
            .await?;

        Ok(messages)
    }
}
