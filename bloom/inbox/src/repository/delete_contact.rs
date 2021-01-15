use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn delete_contact<'c, C: Queryer<'c>>(&self, db: C, contact_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM inbox_contacts WHERE id = $1";

        match sqlx::query(QUERY).bind(contact_id).execute(db).await {
            Err(err) => {
                error!("inbox.delete_contact: Deleting contact: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
