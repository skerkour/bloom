use super::Service;
use crate::{db::Queryer, entities::Upload};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_upload<'c, C: Queryer<'c>>(&self, db: C, upload_id: Uuid) -> Result<Upload, crate::Error> {
        let upload = self.repo.find_upload_by_id(db, upload_id).await?;
        Ok(upload)
    }
}
