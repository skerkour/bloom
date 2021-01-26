use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_sessions_by_user_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        user_id: Uuid,
    ) -> Result<Vec<entities::Session>, Error> {
        const QUERY: &str = "SELECT * FROM kernel_sessions WHERE user_id = $1";

        match sqlx::query_as::<_, entities::Session>(QUERY)
            .bind(user_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_sessions_by_user_id: finding sessions: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
