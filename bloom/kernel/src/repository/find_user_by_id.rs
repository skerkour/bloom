use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_user_by_id<'c, C: Queryer<'c>>(&self, db: C, user_id: Uuid) -> Result<entities::User, Error> {
        const QUERY: &str = "SELECT * FROM kernel_users WHERE id = $1";

        match sqlx::query_as::<_, entities::User>(QUERY)
            .bind(user_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_user_by_id: finding user: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::UserNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
