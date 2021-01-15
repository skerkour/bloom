use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contact_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        contact_id: Uuid,
    ) -> Result<entities::Contact, Error> {
        todo!();
    }
}
