use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn delete_event<'c, C: Queryer<'c>>(&self, db: C, event_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM calendar_events WHERE id = $1";

        match sqlx::query(QUERY).bind(event_id).execute(db).await {
            Err(err) => {
                error!("calendar.delete_event: Deleting event: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
