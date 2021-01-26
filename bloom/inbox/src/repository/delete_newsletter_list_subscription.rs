use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn delete_newsletter_list_subscription<'c, C: Queryer<'c>>(
        &self,
        db: C,
        subscription_id: Uuid,
    ) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM newsletter_lists_subscriptions WHERE id = $1";

        match sqlx::query(QUERY).bind(subscription_id).execute(db).await {
            Err(err) => {
                error!(
                    "inbox.delete_newsletter_list_subscription: Deleting subscription: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
