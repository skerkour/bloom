use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn delete_contact<'c, C: Queryer<'c>>(&self, db: C, contact_id: Uuid) -> Result<(), Error> {
        todo!();
    }
}
