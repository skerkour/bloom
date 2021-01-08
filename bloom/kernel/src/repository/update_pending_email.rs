use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_pending_email<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        pending_email: &entities::PendingEmail,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_pending_emails SET
		updated_at = $1, failed_attempts = $2
		WHERE id = $3";

        match sqlx::query(QUERY)
            .bind(pending_email.updated_at)
            .bind(pending_email.failed_attempts)
            .bind(pending_email.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_pending_email: updating pending email: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
