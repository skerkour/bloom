use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_pending_user<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        pending_user: &entities::PendingUser,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_pending_users
            (id, created_at, updated_at, email, failed_attempts, code_hash, username)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        match sqlx::query(QUERY)
            .bind(pending_user.id)
            .bind(pending_user.created_at)
            .bind(pending_user.updated_at)
            .bind(&pending_user.email)
            .bind(pending_user.failed_attempts)
            .bind(&pending_user.code_hash)
            .bind(&pending_user.username)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_pending_user: Inserting pending user: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
