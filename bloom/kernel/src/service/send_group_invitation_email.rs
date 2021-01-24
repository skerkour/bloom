use super::SendGroupInvitationEmailInput;
use crate::{notifications, Error, Service};
use stdx::{log::error, mail};

impl Service {
    pub async fn send_group_invitation_email(&self, input: SendGroupInvitationEmailInput) -> Result<(), Error> {
        let invitation = self
            .repo
            .find_group_invitation_by_id(&self.db, input.invitation_id)
            .await?;
        let invitee = self.repo.find_user_by_id(&self.db, invitation.invitee_id).await?;
        let inviter = self.repo.find_user_by_id(&self.db, invitation.inviter_id).await?;
        let group = self.repo.find_group_by_id(&self.db, invitation.group_id).await?;

        let data = tera::Context::from_serialize(notifications::GroupInvitationEmailData {
            inviter_name: self.xss.escape(&inviter.name),
            inviter_url: self.namespace_url(&inviter.username),
            invitee_name: self.xss.escape(&invitee.name),
            group_name: self.xss.escape(&group.name),
            invitations_url: self.group_invitations_url(),
        })
        .map_err(|err| {
            error!(
                "kernel.send_group_invitation_email: building template context: {}",
                &err
            );
            Error::Internal(err.to_string())
        })?;
        let to = mail::Address {
            name: invitee.name,
            address: inviter.email,
        };

        let subject = format!("Bloom - {} invted you to join the group {}", &inviter.name, &group.name);

        let html = self
            .templates
            .render(notifications::GROUP_INVITATION_EMAIL_TEMPLATE_ID, &data)
            .map_err(|err| {
                error!("kernel.send_group_invitation_email: rendering tempplate: {}", &err);
                Error::Internal(err.to_string())
            })?;

        let email = mail::Email {
            from: self.config.mail.notify_address.clone(),
            to,
            reply_to: None,
            subject,
            html,
        };
        self.mailer.send(email).await?;

        Ok(())
    }
}
