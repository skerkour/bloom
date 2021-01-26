use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_newsletter_list<'c, C: Queryer<'c>>(&self, db: C, list_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM newsletter_lists WHERE id = $1";

        match sqlx::query(QUERY).bind(list_id).execute(db).await {
            Err(err) => {
                error!("inbox.delete_newsletter_list: Deleting list: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
