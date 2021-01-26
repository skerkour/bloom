use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_invitations_for_invitee<'c, C: Queryer<'c>>(
        &self,
        db: C,
        user_id: Uuid,
    ) -> Result<Vec<entities::GroupInvitation>, Error> {
        const QUERY: &str = "SELECT * FROM kernel_group_invitations WHERE invitee_id = $1";

        match sqlx::query_as::<_, entities::GroupInvitation>(QUERY)
            .bind(user_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_invitations_for_user: finding invitations: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
