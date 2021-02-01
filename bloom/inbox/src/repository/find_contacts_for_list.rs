use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contacts_for_list<'c, C: Queryer<'c>>(
        &self,
        db: C,
        list_id: Uuid,
    ) -> Result<Vec<entities::Contact>, Error> {
        const QUERY: &str = "SELECT inbox_contacts.* FROM inbox_contacts
            INNER JOIN newsletter_lists_subscriptions
                ON newsletter_lists_subscriptions.contact_id = inbox_contacts.id
            WHERE newsletter_lists_subscriptions.list_id = $1";

        match sqlx::query_as::<_, entities::Contact>(QUERY)
            .bind(list_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_contacts_for_list: Finding contacts: {}", &err);
                Err(err.into())
            }
            Ok(contacts) => Ok(contacts),
        }
    }
}
