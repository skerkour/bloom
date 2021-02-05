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
        todo!();
    }
}
