use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_subscription_by_contact_id_and_list_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        contact_id: Uuid,
        list_id: Uuid,
    ) -> Result<entities::NewsletterListSubscription, Error> {
        const QUERY: &str = "SELECT * FROM newsletter_lists_subscriptions
            WHERE contact_id = $1 AND list_id = $2";

        match sqlx::query_as::<_, entities::NewsletterListSubscription>(QUERY)
            .bind(contact_id)
            .bind(list_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.find_newsletter_subscription_by_contact_id_and_list_id: finding subscription: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(None) => Err(Error::NewsletterSubscriptionNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
