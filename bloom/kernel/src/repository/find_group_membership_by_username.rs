use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_membership_by_username<'c, C: Queryer<'c>>(
        &self,
        db: C,
        group_id: Uuid,
        username: &str,
    ) -> Result<entities::GroupMembership, Error> {
        const QUERY: &str = "SELECT kernel_groups_members.* FROM kernel_groups_members
		INNER JOIN kernel_users ON users.id = kernel_groups_members.user_id
		WHERE kernel_users.username = $1 AND kernel_groups_members.group_id = $2";

        match sqlx::query_as::<_, entities::GroupMembership>(QUERY)
            .bind(username)
            .bind(group_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_membership_by_username: finding membership: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::GroupMemberNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
