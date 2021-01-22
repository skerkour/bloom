use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_namespace_group_membership<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
        user_id: Uuid,
    ) -> Result<entities::GroupMembership, Error> {
        const QUERY: &str = "SELECT * FROM kernel_groups_members
            INNER JOIN kernel_groups ON kernel_groups.id = kernel_groups_members.group_id
            WHERE kernel_groups_members.user_id = $1 AND kernel_groups.namespace_id = $2";

        match sqlx::query_as::<_, entities::GroupMembership>(QUERY)
            .bind(user_id)
            .bind(namespace_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_namespace_group_membership: finding membership: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::GroupMemberNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
