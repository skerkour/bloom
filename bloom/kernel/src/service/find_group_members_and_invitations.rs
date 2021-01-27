use super::{GroupInvitationWithDetails, GroupWithMembersAndInvitations, Service};
use crate::Actor;

impl Service {
    pub async fn find_group_members_and_invitations(
        &self,
        actor: Actor,
        group_path: String,
    ) -> Result<GroupWithMembersAndInvitations, crate::Error> {
        let actor = self.current_user(actor)?;

        let group = self.repo.find_group_by_path(&self.db, group_path).await?;

        self.check_namespace_membership(&self.db, &actor, group.namespace_id)
            .await?;

        let invitations = self.repo.find_group_invitations_for_group(&self.db, group.id).await?;

        let mut detailed_invits: Vec<GroupInvitationWithDetails> = Vec::with_capacity(invitations.len());

        for invitation in invitations {
            let inviter = self.repo.find_user_by_id(&self.db, invitation.inviter_id).await?;
            let invitee = self.repo.find_user_by_id(&self.db, invitation.invitee_id).await?;

            detailed_invits.push(GroupInvitationWithDetails {
                invitation,
                group: group.clone(),
                inviter,
                invitee,
            });
        }

        let members = self.repo.find_group_members(&self.db, group.id).await?;

        Ok(GroupWithMembersAndInvitations {
            group,
            members,
            invitations: detailed_invits,
        })
    }
}
