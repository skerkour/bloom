use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_files_by_ids<'c, C: Queryer<'c>>(&self, db: C, file_ids: &[Uuid]) -> Result<Vec<File>, Error> {
        todo!();
    }
}
