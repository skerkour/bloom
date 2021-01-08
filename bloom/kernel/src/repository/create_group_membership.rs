use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_group_membership<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        membership: &entities::GroupMembership,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_groups_members
            (joined_at, role, user_id, group_id)
            VALUES ($1, $2, $3, $4)";

        match sqlx::query(QUERY)
            .bind(membership.joined_at)
            .bind(membership.role)
            .bind(membership.user_id)
            .bind(membership.group_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_group_membership: Inserting membership: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
