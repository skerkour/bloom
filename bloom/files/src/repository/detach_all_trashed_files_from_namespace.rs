use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn detach_all_trashed_files_from_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<(), Error> {
        todo!();
    }
}
