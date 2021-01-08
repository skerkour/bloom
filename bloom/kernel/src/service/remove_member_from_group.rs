use super::{RemoveMemberFromGroupInput, Service};
use crate::{consts::GroupRole, errors::kernel::Error, Actor};

impl Service {
    pub async fn remove_member_from_group(
        &self,
        actor: Actor,
        input: RemoveMemberFromGroupInput,
    ) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        // check that user is admin
        let actor_membership = self
            .repo
            .find_group_membership(&self.db, input.group_id, actor.id)
            .await?;

        if actor_membership.role != GroupRole::Administrator {
            return Err(Error::PermissionDenied.into());
        }

        let membership_to_remove = self
            .repo
            .find_group_membership_by_username(&self.db, input.group_id, &input.username)
            .await?;

        let mut tx = self.db.begin().await?;

        self.repo
            .delete_group_membership(&mut tx, &membership_to_remove)
            .await?;

        let admins_count = self.repo.get_group_admins_count(&mut tx, input.group_id).await?;

        if admins_count == 0 {
            return Err(Error::AtLeatOneAdminMustRemainInGroup.into());
        }

        tx.commit().await?;

        Ok(())
    }
}
