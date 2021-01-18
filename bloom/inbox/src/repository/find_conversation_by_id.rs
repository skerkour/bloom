use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_conversation_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        conversation_id: Uuid,
    ) -> Result<entities::Conversation, Error> {
        const QUERY: &str = "SELECT * FROM inbox_conversations
            WHERE id = $1";

        match sqlx::query_as::<_, entities::Conversation>(QUERY)
            .bind(conversation_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_conversation_by_id: finding conversation: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::ConversationNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
