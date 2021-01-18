use super::Repository;
use crate::{entities::Message, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_inbox_message<'c, C: Queryer<'c>>(&self, db: C, message: &Message) -> Result<(), Error> {
        todo!();
    }
}
