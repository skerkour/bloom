use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_event<'c, C: Queryer<'c>>(&self, db: C, event: &Event) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO calendar_events
        (id, created_at, updated_at, title, description, location, start_at, end_at, namespace_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

        match sqlx::query(QUERY)
            .bind(event.id)
            .bind(event.created_at)
            .bind(event.updated_at)
            .bind(&event.title)
            .bind(&event.description)
            .bind(&event.location)
            .bind(event.start_at)
            .bind(event.end_at)
            .bind(event.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("calendar.create_event: Inserting event: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
