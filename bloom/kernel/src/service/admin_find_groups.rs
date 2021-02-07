use super::Service;
use crate::{entities::Group, errors::kernel::Error, Actor};

impl Service {
    pub async fn admin_find_groups(&self, actor: Actor) -> Result<Vec<Group>, crate::Error> {
        let actor = self.current_user(actor)?;

        if !actor.is_admin {
            return Err(Error::PermissionDenied.into());
        }

        let groups = self.repo.find_all_groups(&self.db).await?;

        Ok(groups)
    }
}
