use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_members<'c, C: Queryer<'c>>(
        &self,
        db: C,
        group_id: Uuid,
    ) -> Result<Vec<entities::User>, Error> {
        const QUERY: &str = "SELECT kernel_users.* FROM kernel_users
        INNER JOIN kernel_groups_members ON kernel_groups_members.user_id = kernel_users.id
        WHERE kernel_groups_members.group_id = $1";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(group_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_members: finding members: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
