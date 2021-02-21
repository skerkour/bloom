use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_messages_by_list_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        list_id: Uuid,
    ) -> Result<Vec<entities::NewsletterMessage>, Error> {
        const QUERY: &str = "SELECT * FROM newsletter_messages
            WHERE list_id = $1 ORDER BY updated_at DESC";

        match sqlx::query_as::<_, entities::NewsletterMessage>(QUERY)
            .bind(list_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_newsletter_messages_by_list_id: Finding messages: {}", &err);
                Err(err.into())
            }
            Ok(messages) => Ok(messages),
        }
    }
}
