use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_all_groups<'c, C: Queryer<'c>>(&self, db: C) -> Result<Vec<entities::Group>, Error> {
        const QUERY: &str = "SELECT * FROM kernel_groups ORDER BY id";

        match sqlx::query_as::<_, entities::Group>(QUERY).fetch_all(db).await {
            Err(err) => {
                error!("kernel.find_all_groups: finding groups: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
