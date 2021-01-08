use super::{CancelGroupInvitationInput, Service};
use crate::{consts::GroupRole, errors::kernel::Error, Actor};

impl Service {
    pub async fn cancel_group_invitation(
        &self,
        actor: Actor,
        input: CancelGroupInvitationInput,
    ) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        let invitation = self
            .repo
            .find_group_invitation_by_id(&self.db, input.invitation_id)
            .await?;

        // check that user is admin
        let membership = self
            .repo
            .find_group_membership(&self.db, invitation.group_id, actor.id)
            .await?;

        if membership.role != GroupRole::Administrator {
            return Err(Error::PermissionDenied.into());
        }

        self.repo.delete_group_invitation(&self.db, invitation.id).await?;

        Ok(())
    }
}
