use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_pending_email<'c, C: Queryer<'c>>(&self, db: C, pending_email_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_pending_emails WHERE id = $1";

        match sqlx::query(QUERY).bind(pending_email_id).execute(db).await {
            Err(err) => {
                error!("kernel.delete_pending_email: Deleting pending email: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
