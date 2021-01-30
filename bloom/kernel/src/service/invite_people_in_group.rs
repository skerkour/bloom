use super::{GroupInvitationWithDetails, InvitePeopleInGroupInput, Service};
use crate::{
    consts::BillingPlan,
    consts::{self, GroupRole},
    entities::GroupInvitation,
    errors::kernel::Error,
    Actor,
};
use std::collections::HashSet;
use stdx::{chrono::Utc, log::error, ulid::Ulid};

impl Service {
    pub async fn invite_people_in_group(
        &self,
        actor: Actor,
        input: InvitePeopleInGroupInput,
    ) -> Result<Vec<GroupInvitationWithDetails>, crate::Error> {
        let actor = self.current_user(actor)?;

        let (group, membership) = self
            .find_group_and_membership(&self.db, actor.id, input.group_id)
            .await?;
        let namespace = self.repo.find_namespace_by_id(&self.db, group.namespace_id).await?;

        if membership.role != GroupRole::Administrator {
            return Err(Error::PermissionDenied.into());
        }

        let current_group_members = self.repo.find_group_members(&self.db, input.group_id).await?;
        let current_group_members_count = current_group_members.len();

        let current_group_members: HashSet<String> = current_group_members
            .into_iter()
            .map(|member| member.username)
            .collect();

        let current_invitees = self.repo.find_group_invitees(&self.db, input.group_id).await?;
        let current_group_invitees: HashSet<String> = current_invitees.into_iter().map(|user| user.username).collect();
        let mut dedup_usernames: HashSet<String> = HashSet::new();
        let mut usernames: Vec<String> = Vec::with_capacity(input.usernames.len());

        for username in input.usernames.into_iter() {
            let is_member = current_group_members.contains(&username);
            let is_invitee = current_group_invitees.contains(&username);
            let is_already_in = dedup_usernames.contains(&username);

            if !is_member && !is_invitee && !is_already_in {
                usernames.push(username.clone());
            }
            dedup_usernames.insert(username);
        }

        let usernames_len = usernames.len();
        let users_to_invite = self.repo.find_users_by_usernames(&self.db, usernames).await?;
        let number_of_users_to_invite = users_to_invite.len();

        if number_of_users_to_invite != usernames_len {
            return Err(Error::SomeUsersNotFound.into());
        }

        if !self.config.self_hosted {
            let members_count_after_invites = current_group_members_count + number_of_users_to_invite;
            match namespace.plan {
                BillingPlan::Free if members_count_after_invites > consts::MAX_MEMBERS_PLAN_FREE => {
                    Err(Error::MembersLimitReachedForPlan)
                }
                BillingPlan::Starter if members_count_after_invites > consts::MAX_MEMBERS_PLAN_STARTER => {
                    Err(Error::MembersLimitReachedForPlan)
                }
                BillingPlan::Pro // | BillingPlan::Ultra
                    if members_count_after_invites > consts::MAX_MEMBERS_SOFT_LIMIT =>
                {
                    Err(Error::SoftLimitReached)
                }
                _ => Ok(()),
            }?;
        }

        let now = Utc::now();
        let invitations: Vec<GroupInvitationWithDetails> = users_to_invite
            .into_iter()
            .map(|invitee| {
                let invitation = GroupInvitation {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    group_id: group.id,
                    inviter_id: actor.id,
                    invitee_id: invitee.id,
                };

                GroupInvitationWithDetails {
                    invitation,
                    invitee,
                    inviter: actor.clone(),
                    group: group.clone(),
                }
            })
            .collect();

        // for each invitee, create invitation
        let mut tx = self.db.begin().await?;

        for invitation in invitations.iter() {
            self.repo
                .create_group_invitation(&mut tx, &invitation.invitation)
                .await?;
        }

        tx.commit().await?;

        // for each invitation, send email
        for invitation in invitations.iter() {
            let job = crate::domain::messages::Message::KernelSendGroupInvitationEmail {
                invitation_id: invitation.invitation.id,
            };
            match self.queue.push(job, None).await {
                Err(err) => error!("kernel.invite_people_in_group: queueing invitation email: {}", err),
                Ok(_) => {}
            };
        }

        Ok(invitations)
    }
}
