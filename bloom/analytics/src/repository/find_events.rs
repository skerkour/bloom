use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_events<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Event>, Error> {
        todo!();
    }
}
