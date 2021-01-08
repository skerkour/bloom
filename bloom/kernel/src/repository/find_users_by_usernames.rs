use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_users_by_usernames<'c, C: Queryer<'c>>(
        &self,
        db: C,
        usernames: Vec<String>,
    ) -> Result<Vec<entities::User>, Error> {
        const QUERY: &str = "SELECT * FROM kernel_users WHERE username = ANY($1)";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(&usernames)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_users_by_usernames: finding users: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
