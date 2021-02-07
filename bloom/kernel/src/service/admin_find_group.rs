use super::Service;
use crate::{entities::Group, errors::kernel::Error, Actor};
use stdx::uuid::Uuid;

impl Service {
    pub async fn admin_find_group(&self, actor: Actor, group_id: Uuid) -> Result<Group, crate::Error> {
        let actor = self.current_user(actor)?;

        if !actor.is_admin {
            return Err(Error::PermissionDenied.into());
        }

        let group = self.repo.find_group_by_id(&self.db, group_id).await?;

        Ok(group)
    }
}
