use super::{ConversationWithMessageAndContacts, FindInboxInput};
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn find_inbox(
        &self,
        actor: Actor,
        input: FindInboxInput,
    ) -> Result<Vec<ConversationWithMessageAndContacts>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let conversations = self
            .repo
            .find_inbox_conversations_by_namespace_id(&self.db, input.namespace_id)
            .await?;

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
