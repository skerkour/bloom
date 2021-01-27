use super::Service;
use crate::{entities::Group, Actor};

impl Service {
    pub async fn find_group(&self, actor: Actor, group_path: String) -> Result<Group, crate::Error> {
        let actor = self.current_user(actor)?;

        let group = self.repo.find_group_by_path(&self.db, group_path).await?;

        self.check_namespace_membership(&self.db, &actor, group.namespace_id)
            .await?;

        Ok(group)
    }
}
