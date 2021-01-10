use super::Repository;
use crate::{entities::Page, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_pages<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Page>, Error> {
        todo!();
    }
}
