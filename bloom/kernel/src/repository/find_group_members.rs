use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_members<'c, C: Queryer<'c>>(
        &self,
        db: C,
        group_id: Uuid,
    ) -> Result<Vec<entities::GroupMember>, Error> {
        const QUERY: &str = "SELECT kernel_users.id AS user_id, kernel_users.username AS username,
                kernel_users.name AS name, kernel_users.avatar_id AS avatar_id,
                kernel_groups_members.joined_at AS joined_at, kernel_groups_members.role AS role
            FROM kernel_users
            INNER JOIN kernel_groups_members ON kernel_groups_members.user_id = kernel_users.id
            WHERE kernel_groups_members.group_id = $1";

        match sqlx::query_as::<_, entities::GroupMember>(QUERY)
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
