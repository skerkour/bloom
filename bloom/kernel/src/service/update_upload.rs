use super::Service;
use crate::{db::Queryer, entities::Upload};

impl Service {
    pub async fn update_upload<'c, C: Queryer<'c>>(&self, db: C, upload: &Upload) -> Result<(), crate::Error> {
        self.repo.update_upload(db, upload).await?;
        Ok(())
    }
}
