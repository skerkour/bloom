use super::Service;
use crate::{db::Queryer, entities::Upload};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_upload<'c, C: Queryer<'c>>(&self, _db: C, _upload_id: Uuid) -> Result<Upload, crate::Error> {
        unimplemented!();
    }
}
