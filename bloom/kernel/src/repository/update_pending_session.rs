use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_pending_session<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        pending_session: &entities::PendingSession,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_pending_sessions SET
		updated_at = $1, failed_attempts = $2, two_fa_verified = $3
		WHERE id = $4";

        match sqlx::query(QUERY)
            .bind(pending_session.updated_at)
            .bind(pending_session.failed_attempts)
            .bind(pending_session.two_fa_verified)
            .bind(pending_session.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_pending_session: updating pending session: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
