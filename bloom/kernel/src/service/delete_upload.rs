use super::Service;
use crate::db::Queryer;
use stdx::uuid::Uuid;

impl Service {
    pub async fn delete_upload<'c, C: Queryer<'c>>(&self, db: C, upload_id: Uuid) -> Result<(), crate::Error> {
        self.repo.delete_upload(db, upload_id).await?;
        Ok(())
    }
}
