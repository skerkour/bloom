use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_newsletter_message<'c, C: Queryer<'c>>(&self, db: C, message_id: Uuid) -> Result<(), Error> {
        todo!();
    }
}
