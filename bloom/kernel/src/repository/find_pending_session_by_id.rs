use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_pending_session_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        pending_session_id: Uuid,
    ) -> Result<entities::PendingSession, Error> {
        const QUERY: &str = "SELECT * FROM kernel_pending_sessions WHERE id = $1";

        match sqlx::query_as::<_, entities::PendingSession>(QUERY)
            .bind(pending_session_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_pending_session_by_id: finding pending session: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::SessionNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
