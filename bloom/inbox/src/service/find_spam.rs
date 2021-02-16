use super::{ConversationWithMessageAndContacts, FindSpamInput};
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn find_spam(
        &self,
        actor: Actor,
        input: FindSpamInput,
    ) -> Result<Vec<ConversationWithMessageAndContacts>, kernel::Error> {
        // TODO: messages
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let conversations = self.repo.find_spam_conversations(&self.db, input.namespace_id).await?;

        let mut ret = Vec::with_capacity(conversations.len());

        // TODO: batch...
        for conversation in conversations {
            let messages = self
                .repo
                .find_inbox_messages_for_conversation(&self.db, conversation.id, input.after)
                .await?;
            // TODO
            let contacts = Vec::new();
            ret.push(ConversationWithMessageAndContacts {
                conversation,
                messages,
                contacts,
            });
        }

        Ok(ret)
    }
}
