use crate::{Error, Service};
use kernel::Actor;
use stdx::{chrono::Utc, uuid::Uuid};

impl Service {
    pub async fn move_conversation_to_archive(&self, actor: Actor, conversation_id: Uuid) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let mut conversation = self.repo.find_conversation_by_id(&self.db, conversation_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, conversation.namespace_id)
            .await
            .map_err(|_| Error::ConversationNotFound)?;

        let now = Utc::now();
        conversation.updated_at = now;
        conversation.trashed_at = None;
        conversation.archived_at = Some(now);
        conversation.is_spam = false;
        self.repo.update_conversation(&self.db, &conversation).await?;

        Ok(())
    }
}
