use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{
    chrono::{DateTime, Utc},
    log::error,
    sqlx,
    uuid::Uuid,
};

impl Repository {
    pub async fn find_events<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Event>, Error> {
        const QUERY: &str = "SELECT * FROM calendar_events
            WHERE namespace_id = $1
                AND (start_at BETWEEN $2 AND $3
                    OR end_at BETWEEN $2 AND $3)";

        match sqlx::query_as::<_, Event>(QUERY)
            .bind(namespace_id)
            .bind(from)
            .bind(to)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("calendar.find_events: Finding events: {}", &err);
                Err(err.into())
            }
            Ok(files) => Ok(files),
        }
    }
}
