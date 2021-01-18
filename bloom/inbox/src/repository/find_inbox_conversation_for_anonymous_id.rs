use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_inbox_conversation_for_anonymous_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        anonymous_id: Uuid,
        namespace_id: Uuid,
    ) -> Result<entities::Conversation, Error> {
        const QUERY: &str = "SELECT * FROM inbox_conversations
            INNER JOIN inbox_conversations_anonymous_ids ON inbox_conversations_anonymous_ids.conversation_id = inbox_conversations.id
            WHERE inbox_conversations_anonymous_ids.anonymous_id = $1
                AND inbox_conversations.namespace_id = $2";

        match sqlx::query_as::<_, entities::Conversation>(QUERY)
            .bind(anonymous_id)
            .bind(namespace_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.find_inbox_conversation_for_anonymous_id: finding conversation: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(None) => Err(Error::ConversationNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
