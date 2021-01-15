use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_newsletter_list<'c, C: Queryer<'c>>(&self, db: C, list_id: Uuid) -> Result<(), Error> {
        todo!();
    }
}
