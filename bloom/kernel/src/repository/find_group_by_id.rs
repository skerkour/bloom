use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_group_by_id<'c, C: Queryer<'c>>(&self, db: C, group_id: Uuid) -> Result<entities::Group, Error> {
        const QUERY: &str = "SELECT * FROM kernel_groups WHERE id = $1";

        match sqlx::query_as::<_, entities::Group>(QUERY)
            .bind(group_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_by_id: finding group: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::GroupNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
