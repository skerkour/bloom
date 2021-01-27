use super::Repository;
use crate::{entities::NewsletterMessage, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_newsletter_message<'c, C: Queryer<'c>>(
        &self,
        db: C,
        message: &NewsletterMessage,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO newsletter_messages
            (id, created_at, updated_at, name, subject, body, body_html, status, scheduled_for, last_sent_at,
                sent_count, error_count, list_id, namespace_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)";

        match sqlx::query(QUERY)
            .bind(message.id)
            .bind(message.created_at)
            .bind(message.updated_at)
            .bind(&message.name)
            .bind(&message.subject)
            .bind(&message.body)
            .bind(&message.body_html)
            .bind(message.status)
            .bind(message.scheduled_for)
            .bind(message.last_sent_at)
            .bind(message.sent_count)
            .bind(message.error_count)
            .bind(message.list_id)
            .bind(message.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("inbox.create_newsletter_message: Inserting message: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
