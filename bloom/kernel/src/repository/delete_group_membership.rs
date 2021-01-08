use super::Repository;
use crate::{db::Queryer, entities::GroupMembership, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn delete_group_membership<'c, C: Queryer<'c>>(
        &self,
        db: C,
        group_membership: &GroupMembership,
    ) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_groups_members WHERE group_id = $1 AND user_id = $2";

        match sqlx::query(QUERY)
            .bind(group_membership.group_id)
            .bind(group_membership.user_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.kernel_groups_members: Deleting group member: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
