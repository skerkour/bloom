use super::Repository;
use crate::{entities::Visitor, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_visitor<'c, C: Queryer<'c>>(&self, db: C, visitor: &Visitor) -> Result<(), Error> {
        todo!();
    }
}
