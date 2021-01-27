use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_group_by_path<'c, C: Queryer<'c>>(&self, db: C, path: String) -> Result<entities::Group, Error> {
        const QUERY: &str = "SELECT * FROM kernel_groups WHERE path = $1";

        match sqlx::query_as::<_, entities::Group>(QUERY)
            .bind(&path)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_group_by_path: finding group: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::GroupNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
