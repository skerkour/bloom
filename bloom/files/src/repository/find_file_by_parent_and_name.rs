use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_file_by_parent_and_name<'c, C: Queryer<'c>>(
        &self,
        db: C,
        parent_id: Uuid,
        name: &str,
    ) -> Result<entities::File, Error> {
        todo!();
    }
}
