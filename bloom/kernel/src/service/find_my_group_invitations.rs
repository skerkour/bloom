use super::{GroupInvitationWithDetails, Service};
use crate::Actor;

impl Service {
    pub async fn find_my_group_invitations(
        &self,
        actor: Actor,
    ) -> Result<Vec<GroupInvitationWithDetails>, crate::Error> {
        let actor = self.current_user(actor)?;

        let invitations = self.repo.find_group_invitations_for_invitee(&self.db, actor.id).await?;

        let mut ret: Vec<GroupInvitationWithDetails> = Vec::with_capacity(invitations.len());

        for invitation in invitations {
            let inviter = self.repo.find_user_by_id(&self.db, invitation.inviter_id).await?;
            let group = self.repo.find_group_by_id(&self.db, invitation.group_id).await?;

            ret.push(GroupInvitationWithDetails {
                invitation,
                group,
                inviter,
                invitee: actor.clone(),
            });
        }

        Ok(ret)
    }
}
