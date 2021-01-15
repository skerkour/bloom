use super::FindArchiveInput;
use crate::{entities::Conversation, Service};
use kernel::Actor;

impl Service {
    // TODO: messages
    pub async fn find_archive(
        &self,
        actor: Actor,
        input: FindArchiveInput,
    ) -> Result<Vec<Conversation>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, input.namespace_id)
            .await?;

        let conversations = self
            .repo
            .find_archived_conversations(&self.db, input.namespace_id)
            .await?;
        Ok(conversations)
    }
}
