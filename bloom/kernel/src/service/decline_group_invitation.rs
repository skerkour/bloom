use super::{DeclineGroupInvitationInput, Service};
use crate::{errors::kernel::Error, Actor};

impl Service {
    pub async fn decline_group_invitation(
        &self,
        actor: Actor,
        input: DeclineGroupInvitationInput,
    ) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        let invitation = self
            .repo
            .find_group_invitation_by_id(&self.db, input.invitation_id)
            .await?;

        if invitation.invitee_id != actor.id {
            return Err(Error::PermissionDenied.into());
        }

        self.repo.delete_group_invitation(&self.db, invitation.id).await?;

        Ok(())
    }
}
