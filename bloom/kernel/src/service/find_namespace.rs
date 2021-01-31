use super::Service;
use crate::{db::DB, entities::Namespace};
use stdx::uuid::Uuid;

impl Service {
    /// Warning, this method does not check for authentication / authorization.
    pub async fn find_namespace(&self, db: &DB, namespace_id: Uuid) -> Result<Namespace, crate::Error> {
        let namespace = self.repo.find_namespace_by_id(db, namespace_id).await?;

        Ok(namespace)
    }
}
