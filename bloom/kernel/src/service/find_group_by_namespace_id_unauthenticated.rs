use super::Service;
use crate::{db::Queryer, entities::Group};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_group_by_namespace_id_unauthenticated<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Group, crate::Error> {
        let group = self.repo.find_group_by_namespace_id(db, namespace_id).await?;
        Ok(group)
    }
}
