use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_membership<'c, C: Queryer<'c>>(
        &self,
        db: C,
        group_id: Uuid,
        user_id: Uuid,
    ) -> Result<entities::GroupMembership, Error> {
        const QUERY: &str = "SELECT * FROM kernel_groups_members WHERE user_id = $1 AND group_id = $2";

        match sqlx::query_as::<_, entities::GroupMembership>(QUERY)
            .bind(user_id)
            .bind(group_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_membership: finding membership: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::GroupMemberNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
