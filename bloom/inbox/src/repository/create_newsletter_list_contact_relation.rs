use super::Repository;
use crate::{entities::NewsletterListContactRelation, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_newsletter_list_contact_relation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        list: &NewsletterListContactRelation,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_newsletter_lists_contacts
            (list_id, contact_id)
            VALUES ($1, $2)";

        match sqlx::query(QUERY)
            .bind(list.list_id)
            .bind(list.contact_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.create_newsletter_list_contact_relation: Inserting relation: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
