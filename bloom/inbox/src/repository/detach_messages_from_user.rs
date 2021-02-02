use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::{chrono::Utc, log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn detach_messages_from_user<'c, C: Queryer<'c>>(&self, db: C, user_id: Uuid) -> Result<(), Error> {
        let now = Utc::now();
        const QUERY: &str = "UPDATE inbox_messages SET
            updated_at = $1, author_id = NULL
        WHERE author_id = $2";

        match sqlx::query(QUERY).bind(now).bind(user_id).execute(db).await {
            Err(err) => {
                error!("inbox.detach_messages_from_user: Updating messages: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
