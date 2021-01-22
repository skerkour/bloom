use super::FindArchiveInput;
use crate::{entities::Conversation, Service};
use kernel::Actor;

impl Service {
    pub async fn find_archive(
        &self,
        actor: Actor,
        input: FindArchiveInput,
    ) -> Result<Vec<Conversation>, kernel::Error> {
        // TODO: messages
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let conversations = self
            .repo
            .find_archived_conversations(&self.db, input.namespace_id)
            .await?;
        Ok(conversations)
    }
}
