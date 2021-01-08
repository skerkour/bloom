use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_pending_email<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        pending_email: &entities::PendingEmail,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_pending_emails
            (id, created_at, updated_at, email, code_hash, failed_attempts, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        match sqlx::query(QUERY)
            .bind(pending_email.id)
            .bind(pending_email.created_at)
            .bind(pending_email.updated_at)
            .bind(&pending_email.email)
            .bind(&pending_email.code_hash)
            .bind(pending_email.failed_attempts)
            .bind(pending_email.user_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_pending_email: Inserting pending email: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
