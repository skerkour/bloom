use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_group_invitation<'c, C: Queryer<'c>>(&self, db: C, invitation_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_group_invitations WHERE id = $1";

        match sqlx::query(QUERY).bind(invitation_id).execute(db).await {
            Err(err) => {
                error!("kernel.delete_group_invitation: Deleting group invitation: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
