use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_session<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        session: &entities::Session,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_sessions
            (id, created_at, updated_at, secret_hash, user_id)
            VALUES ($1, $2, $3, $4, $5)";

        match sqlx::query(QUERY)
            .bind(session.id)
            .bind(session.created_at)
            .bind(session.updated_at)
            .bind(&session.secret_hash)
            .bind(session.user_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_session: Inserting session: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
