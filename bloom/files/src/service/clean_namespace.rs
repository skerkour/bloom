use crate::Service;
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Service {
    pub async fn clean_namespace<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<(), crate::Error> {
        self.repo.detach_all_files_from_namespace(db, namespace_id).await?;
        Ok(())
    }
}
