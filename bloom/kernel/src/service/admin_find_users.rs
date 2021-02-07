use super::Service;
use crate::{entities::User, errors::kernel::Error, Actor};

impl Service {
    pub async fn admin_find_users(&self, actor: Actor) -> Result<Vec<User>, crate::Error> {
        let actor = self.current_user(actor)?;

        if !actor.is_admin {
            return Err(Error::PermissionDenied.into());
        }

        let users = self.repo.find_all_users(&self.db).await?;

        Ok(users)
    }
}
