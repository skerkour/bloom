use super::Repository;
use crate::{consts::GroupRole, db::Queryer, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn get_group_admins_count<'c, C: Queryer<'c>>(&self, db: C, group_id: Uuid) -> Result<i64, Error> {
        const QUERY: &str = "SELECT COUNT(*) FROM kernel_groups_members WHERE group_id = $1 AND role = $2";

        let (count,): (i64,) = match sqlx::query_as(QUERY)
            .bind(group_id)
            .bind(GroupRole::Administrator)
            .fetch_one(db)
            .await
        {
            Err(err) => {
                error!("kernel.get_group_admins_count: finding admins count: {}", &err);
                Err(err)
            }
            Ok(res) => Ok(res),
        }?;
        Ok(count)
    }
}
