use super::Repository;
use crate::{entities::Contact, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_contact<'c, C: Queryer<'c>>(&self, db: C, contact: &Contact) -> Result<(), Error> {
        todo!();
    }
}
