use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn delete_event<'c, C: Queryer<'c>>(&self, db: C, event_id: Uuid) -> Result<(), Error> {
        todo!();
    }
}
