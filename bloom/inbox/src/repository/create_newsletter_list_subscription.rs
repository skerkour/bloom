use super::Repository;
use crate::{entities::NewsletterListSubscription, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_newsletter_list_subscription<'c, C: Queryer<'c>>(
        &self,
        db: C,
        subscription: &NewsletterListSubscription,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO newsletter_lists_subscriptions
            (id, created_at, updated_at, list_id, contact_id)
            VALUES ($1, $2, $3, $4, $5)";

        match sqlx::query(QUERY)
            .bind(subscription.id)
            .bind(subscription.created_at)
            .bind(subscription.updated_at)
            .bind(subscription.list_id)
            .bind(subscription.contact_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.create_newsletter_list_subscription: Inserting subscription: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
