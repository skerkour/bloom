use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_children_recursively<'c, C: Queryer<'c>>(
        &self,
        db: C,
        file_id: Uuid,
    ) -> Result<Vec<File>, Error> {
        todo!();
    }
}
