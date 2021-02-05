use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_event<'c, C: Queryer<'c>>(&self, db: C, event: &Event) -> Result<(), Error> {
        todo!();
    }
}
