use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn get_users_count<'c, C: Queryer<'c>>(&self, db: C) -> Result<i64, Error> {
        const QUERY: &str = "SELECT COUNT(id) FROM kernel_users";

        let (count,): (i64,) = match sqlx::query_as(QUERY).fetch_one(db).await {
            Err(err) => {
                error!("kernel.get_users_count: finding users count: {}", &err);
                Err(err)
            }
            Ok(res) => Ok(res),
        }?;
        Ok(count)
    }
}
