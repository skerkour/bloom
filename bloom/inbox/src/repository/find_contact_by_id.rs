use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contact_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        contact_id: Uuid,
    ) -> Result<entities::Contact, Error> {
        const QUERY: &str = "SELECT * FROM inbox_contacts
            WHERE id = $1";

        match sqlx::query_as::<_, entities::Contact>(QUERY)
            .bind(contact_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_contact_by_id: finding contact: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::ContactNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
