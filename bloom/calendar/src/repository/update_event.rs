use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_event<'c, C: Queryer<'c>>(&self, db: C, event: &Event) -> Result<(), Error> {
        const QUERY: &str = "UPDATE calendar_events SET
            updated_at = $1, title = $2, description = $3, location = $4, start_at = $5, end_at = $6
            WHERE id = $7";

        match sqlx::query(QUERY)
            .bind(event.updated_at)
            .bind(&event.title)
            .bind(&event.description)
            .bind(&event.location)
            .bind(event.start_at)
            .bind(event.end_at)
            .bind(event.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("calendar.update_event: Updating event: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
