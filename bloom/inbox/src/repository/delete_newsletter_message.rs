use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_newsletter_message<'c, C: Queryer<'c>>(&self, db: C, message_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM newsletter_messages WHERE id = $1";

        match sqlx::query(QUERY).bind(message_id).execute(db).await {
            Err(err) => {
                error!("inbox.delete_newsletter_message: Deleting message: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
