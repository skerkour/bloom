use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_event_by_id<'c, C: Queryer<'c>>(&self, db: C, event_id: Uuid) -> Result<Event, Error> {
        const QUERY: &str = "SELECT * FROM calendar_events
            WHERE id = $1";

        match sqlx::query_as::<_, Event>(QUERY)
            .bind(event_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("calendar.find_event_by_id: finding event: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::EventNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
