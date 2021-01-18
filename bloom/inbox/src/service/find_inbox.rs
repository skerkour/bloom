use super::FindInboxInput;
use crate::{entities::Conversation, Service};
use kernel::Actor;

impl Service {
    pub async fn find_inbox(&self, actor: Actor, input: FindInboxInput) -> Result<Vec<Conversation>, kernel::Error> {
        // TODO: messages
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, input.namespace_id)
            .await?;

        let conversations = self
            .repo
            .find_inbox_conversations_by_namespace_id(&self.db, input.namespace_id)
            .await?;
        Ok(conversations)
    }
}
