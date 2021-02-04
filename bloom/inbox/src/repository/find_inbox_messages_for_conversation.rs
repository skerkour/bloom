use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_inbox_messages_for_conversation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        conversation_id: Uuid,
    ) -> Result<Vec<entities::Message>, Error> {
        const QUERY: &str = "SELECT * FROM inbox_messages
            WHERE conversation_id = $1
            ORDER BY id";

        match sqlx::query_as::<_, entities::Message>(QUERY)
            .bind(conversation_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_inbox_messages_for_conversation: Finding messages: {}", &err);
                Err(err.into())
            }
            Ok(messages) => Ok(messages),
        }
    }
}
