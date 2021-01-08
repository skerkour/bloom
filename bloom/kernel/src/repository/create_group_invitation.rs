use super::Repository;
use crate::{db::Queryer, entities::GroupInvitation, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_group_invitation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        invitation: &GroupInvitation,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_group_invitations
            (id, created_at, updated_at, group_id, inviter_id, invitee_id)
            VALUES ($1, $2, $3, $4, $5, $6)";

        match sqlx::query(QUERY)
            .bind(invitation.id)
            .bind(invitation.created_at)
            .bind(invitation.updated_at)
            .bind(invitation.group_id)
            .bind(invitation.inviter_id)
            .bind(invitation.invitee_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_group_invitation: Inserting group invitation: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
