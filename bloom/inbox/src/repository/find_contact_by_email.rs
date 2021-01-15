use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contact_by_email<'c, C: Queryer<'c>>(
        &self,
        db: C,
        email: &str,
    ) -> Result<entities::Contact, Error> {
        todo!();
    }
}
