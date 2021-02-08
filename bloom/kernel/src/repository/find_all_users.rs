use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_all_users<'c, C: Queryer<'c>>(&self, db: C) -> Result<Vec<entities::User>, Error> {
        const QUERY: &str = "SELECT * FROM kernel_users ORDER BY id";

        match sqlx::query_as::<_, entities::User>(QUERY).fetch_all(db).await {
            Err(err) => {
                error!("kernel.find_all_users: finding users: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
