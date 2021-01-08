use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_user_by_email<'c, C: Queryer<'c>>(&self, db: C, email: &str) -> Result<entities::User, Error> {
        const QUERY: &str = "SELECT * FROM kernel_users WHERE email = $1";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(email)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_user_by_email: finding user: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::UserNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
