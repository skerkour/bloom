use super::Repository;
use crate::{entities::Visit, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_visits<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Visit>, Error> {
        todo!();
    }
}
