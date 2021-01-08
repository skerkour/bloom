use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_pending_session<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        pending_session: &entities::PendingSession,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_pending_sessions
            (id, created_at, updated_at, code_hash, user_id, failed_attempts, two_fa_verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        match sqlx::query(QUERY)
            .bind(pending_session.id)
            .bind(pending_session.created_at)
            .bind(pending_session.updated_at)
            .bind(&pending_session.code_hash)
            .bind(&pending_session.user_id)
            .bind(pending_session.failed_attempts)
            .bind(pending_session.two_fa_verified)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_pending_session: Inserting pending session: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
