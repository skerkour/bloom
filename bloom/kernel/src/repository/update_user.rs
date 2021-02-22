use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_user<'c, C: db::Queryer<'c>>(&self, db: C, user: &entities::User) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_users SET
        updated_at = $1, blocked_at = $2, email = $3, username = $4, name = $5, description = $6,
        is_admin = $7, avatar_id = $8, two_fa_enabled = $9, two_fa_method = $10, encrypted_totp_secret = $11,
        totp_secret_nonce = $12
		WHERE id = $13";

        match sqlx::query(QUERY)
            .bind(user.updated_at)
            .bind(user.blocked_at)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.name)
            .bind(&user.description)
            .bind(user.is_admin)
            .bind(&user.avatar_id)
            .bind(user.two_fa_enabled)
            .bind(user.two_fa_method)
            .bind(&user.encrypted_totp_secret)
            .bind(&user.totp_secret_nonce)
            .bind(user.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_user: updating user: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
