use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_pending_email_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        pending_email_id: Uuid,
    ) -> Result<entities::PendingEmail, Error> {
        const QUERY: &str = "SELECT * FROM kernel_pending_emails WHERE id = $1";

        match sqlx::query_as::<_, entities::PendingEmail>(QUERY)
            .bind(pending_email_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_pending_email_by_id: finding pending email: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::PendingEmailNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
