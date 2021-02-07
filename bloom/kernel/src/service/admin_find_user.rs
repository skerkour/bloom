use super::Service;
use crate::{entities::User, errors::kernel::Error, Actor};
use stdx::uuid::Uuid;

impl Service {
    pub async fn admin_find_user(&self, actor: Actor, user_id: Uuid) -> Result<User, crate::Error> {
        let actor = self.current_user(actor)?;

        if !actor.is_admin {
            return Err(Error::PermissionDenied.into());
        }

        let user = self.repo.find_user_by_id(&self.db, user_id).await?;

        Ok(user)
    }
}
