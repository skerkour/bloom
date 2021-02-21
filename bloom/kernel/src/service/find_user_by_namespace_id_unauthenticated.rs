use super::Service;
use crate::{db::Queryer, entities::User};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_user_by_namespace_id_unauthenticated<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<User, crate::Error> {
        let user = self.repo.find_user_by_namespace_id(db, namespace_id).await?;
        Ok(user)
    }
}
