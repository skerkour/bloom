use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_message_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        message_id: Uuid,
    ) -> Result<entities::NewsletterMessage, Error> {
        const QUERY: &str = "SELECT * FROM newsletter_messages
            WHERE id = $1";

        match sqlx::query_as::<_, entities::NewsletterMessage>(QUERY)
            .bind(message_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_newsletter_message_by_id: finding message: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::NewsletterMessageNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
