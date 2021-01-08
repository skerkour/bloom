use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_invitation_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        invitation_id: Uuid,
    ) -> Result<entities::GroupInvitation, Error> {
        const QUERY: &str = "SELECT * FROM kernel_group_invitations WHERE id = $1";

        match sqlx::query_as::<_, entities::GroupInvitation>(QUERY)
            .bind(invitation_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_invitation_by_id: finding invitation: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::GroupNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
