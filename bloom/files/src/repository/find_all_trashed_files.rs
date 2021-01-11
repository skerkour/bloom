use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_all_trashed_files<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<File>, Error> {
        todo!();
    }
}
