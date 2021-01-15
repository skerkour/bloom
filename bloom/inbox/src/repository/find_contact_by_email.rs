use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contact_by_email<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
        email: &str,
    ) -> Result<entities::Contact, Error> {
        const QUERY: &str = "SELECT * FROM inbox_contacts
            WHERE namespace_id = $1 AND email = $2";

        match sqlx::query_as::<_, entities::Contact>(QUERY)
            .bind(namespace_id)
            .bind(email)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_contact_by_email: finding contact: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::ContactNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
