use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contact_by_anonymous_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        anonymous_id: Uuid,
        namespace_id: Uuid,
    ) -> Result<entities::Contact, Error> {
        const QUERY: &str = "SELECT * FROM inbox_contacts
            INNER JOIN inbox_contacts_anonymous_ids ON inbox_contacts_anonymous_ids.contact_id = inbox_contacts.id
            WHERE inbox_contacts_anonymous_ids.anonymous_id = $1
                AND inbox_contacts.namespace_id = $2";

        match sqlx::query_as::<_, entities::Contact>(QUERY)
            .bind(anonymous_id)
            .bind(namespace_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_contact_by_anonymous_id: finding contact: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::ContactNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
