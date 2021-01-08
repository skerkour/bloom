use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_pending_user<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        pending_user: &entities::PendingUser,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_pending_users SET
		updated_at = $1, failed_attempts = $2
		WHERE id = $3";

        match sqlx::query(QUERY)
            .bind(pending_user.updated_at)
            .bind(pending_user.failed_attempts)
            .bind(pending_user.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_pending_user: updating pending user: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
