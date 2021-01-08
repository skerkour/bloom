use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_invitees<'c, C: Queryer<'c>>(
        &self,
        db: C,
        group_id: Uuid,
    ) -> Result<Vec<entities::User>, Error> {
        const QUERY: &str = "SELECT kernel_users.* FROM kernel_users
        INNER JOIN kernel_group_invitations ON kernel_group_invitations.invitee_id = kernel_users.id
        WHERE kernel_group_invitations.group_id = $1";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(group_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_invitees: finding invitees: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
