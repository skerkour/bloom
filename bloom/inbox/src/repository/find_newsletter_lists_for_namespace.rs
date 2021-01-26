use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_lists_for_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<entities::NewsletterList>, Error> {
        const QUERY: &str = "SELECT * FROM newsletter_lists
            WHERE namespace_id = $1 ORDER BY id";

        match sqlx::query_as::<_, entities::NewsletterList>(QUERY)
            .bind(namespace_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_newsletter_lists_for_namespace: Finding lists: {}", &err);
                Err(err.into())
            }
            Ok(lists) => Ok(lists),
        }
    }
}
