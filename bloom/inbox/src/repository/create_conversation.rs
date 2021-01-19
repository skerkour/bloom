use super::Repository;
use crate::{entities::Conversation, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_conversation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        conversation: &Conversation,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_conversations
        (id, created_at, updated_at, archived_at, trashed_at, last_message_at, is_spam, name,
            description, anonymous_id, namespace_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";

        match sqlx::query(QUERY)
            .bind(conversation.id)
            .bind(conversation.created_at)
            .bind(conversation.updated_at)
            .bind(conversation.archived_at)
            .bind(conversation.trashed_at)
            .bind(conversation.last_message_at)
            .bind(conversation.is_spam)
            .bind(&conversation.name)
            .bind(&conversation.description)
            .bind(conversation.anonymous_id)
            .bind(conversation.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("inbox.create_conversation: Inserting conversation: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
