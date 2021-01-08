use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_session<'c, C: Queryer<'c>>(&self, db: C, session_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_sessions WHERE id = $1";

        match sqlx::query(QUERY).bind(session_id).execute(db).await {
            Err(err) => {
                error!("kernel.delete_session: Deleting session: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
