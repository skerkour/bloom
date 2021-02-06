use super::Service;
use crate::{entities::User, errors::kernel::Error, Actor};
use stdx::{chrono::Utc, uuid::Uuid};

impl Service {
    pub async fn admin_unblock_user(&self, actor: Actor, user_id: Uuid) -> Result<User, crate::Error> {
        let actor = self.current_user(actor)?;

        if !actor.is_admin {
            return Err(Error::PermissionDenied.into());
        }

        let mut user = self.repo.find_user_by_id(&self.db, user_id).await?;

        let now = Utc::now();
        user.updated_at = now;
        user.blocked_at = None;

        self.repo.update_user(&self.db, &user).await?;

        Ok(user)
    }
}
