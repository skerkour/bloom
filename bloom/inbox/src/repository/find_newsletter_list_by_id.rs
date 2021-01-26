use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_list_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        list_id: Uuid,
    ) -> Result<entities::NewsletterList, Error> {
        const QUERY: &str = "SELECT * FROM newsletter_lists
            WHERE id = $1";

        match sqlx::query_as::<_, entities::NewsletterList>(QUERY)
            .bind(list_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_newsletter_list_by_id: finding list: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::NewsletterListNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
