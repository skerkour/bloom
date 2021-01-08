use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_pending_user_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        pending_user_id: Uuid,
    ) -> Result<entities::PendingUser, Error> {
        const QUERY: &str = "SELECT * FROM kernel_pending_users WHERE id = $1";

        match sqlx::query_as::<_, entities::PendingUser>(QUERY)
            .bind(pending_user_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_pending_user_by_id: finding pending user: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::UserNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
