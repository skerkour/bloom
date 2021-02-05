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
    pub async fn find_event_by_id<'c, C: Queryer<'c>>(&self, db: C, event_id: Uuid) -> Result<Event, Error> {
        todo!();
    }
}
