use super::Repository;
use crate::{entities::NewsletterMessage, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_newsletter_message<'c, C: Queryer<'c>>(
        &self,
        db: C,
        message: &NewsletterMessage,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE newsletter_messages SET
            updated_at = $1, name = $2, subject = $3, body = $4, body_html = $5, status = $6, scheduled_for = $7,
            last_sent_at = $8, sent_count = $9, error_count = $10, list_id = $11
            WHERE id = $12";

        match sqlx::query(QUERY)
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
            .bind(message.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.update_newsletter_message: Updating message: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
