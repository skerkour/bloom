use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::log::error;
use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
};

impl Repository {
    pub async fn delete_old_pending_sessions<'c, C: Queryer<'c>>(
        &self,
        db: C,
        older_than: DateTime<Utc>,
    ) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_pending_sessions WHERE created_at < $1";

        match sqlx::query(QUERY).bind(older_than).execute(db).await {
            Err(err) => {
                error!(
                    "kernel.delete_old_pending_sessions: Deleting pending sessions: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
