use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contacts_for_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<entities::Contact>, Error> {
        const QUERY: &str = "SELECT * FROM inbox_contacts
            WHERE namespace_id = $1 ORDER BY name";

        match sqlx::query_as::<_, entities::Contact>(QUERY)
            .bind(namespace_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_contacts_for_namespace: Finding contacts: {}", &err);
                Err(err.into())
            }
            Ok(contacts) => Ok(contacts),
        }
    }
}
