use super::Repository;
use crate::{entities::Message, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_inbox_message<'c, C: Queryer<'c>>(&self, db: C, message: &Message) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_messages
        (id, created_at, updated_at, received_at, body_html, conversation_id, author_id, from_operator)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

        match sqlx::query(QUERY)
            .bind(message.id)
            .bind(message.created_at)
            .bind(message.updated_at)
            .bind(message.received_at)
            .bind(&message.body_html)
            .bind(message.conversation_id)
            .bind(message.author_id)
            .bind(message.from_operator)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("inbox.create_inbox_message: Inserting message: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
