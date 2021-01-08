use super::{AcceptGroupInvitationInput, Service};
use crate::{
    consts::GroupRole,
    entities::{Group, GroupMembership},
    errors::kernel::Error,
    Actor,
};
use stdx::chrono::Utc;

impl Service {
    pub async fn accept_group_invitation(
        &self,
        actor: Actor,
        input: AcceptGroupInvitationInput,
    ) -> Result<Group, crate::Error> {
        let actor = self.current_user(actor)?;

        let mut tx = self.db.begin().await?;

        let invitation = self
            .repo
            .find_group_invitation_by_id(&mut tx, input.invitation_id)
            .await?;

        if invitation.invitee_id != actor.id {
            return Err(Error::PermissionDenied.into());
        }

        let now = Utc::now();

        // create group membership, delete invitation
        let membership = GroupMembership {
            joined_at: now,
            role: GroupRole::Member,
            user_id: actor.id,
            group_id: invitation.group_id,
        };
        self.repo.create_group_membership(&mut tx, &membership).await?;

        self.repo.delete_group_invitation(&mut tx, invitation.id).await?;

        tx.commit().await?;

        let group = self.repo.find_group_by_id(&self.db, invitation.group_id).await?;
        Ok(group)
    }
}
