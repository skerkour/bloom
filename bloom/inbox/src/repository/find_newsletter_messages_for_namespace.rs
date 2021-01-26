use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_messages_for_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<entities::NewsletterMessage>, Error> {
        const QUERY: &str = "SELECT * FROM newsletter_messages
            WHERE namespace_id = $1 ORDER BY updated_at";

        match sqlx::query_as::<_, entities::NewsletterMessage>(QUERY)
            .bind(namespace_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.find_newsletter_messages_for_namespace: Finding messages: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(messages) => Ok(messages),
        }
    }
}
