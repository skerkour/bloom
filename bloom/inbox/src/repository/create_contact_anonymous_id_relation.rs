use super::Repository;
use crate::{entities::ContactAnonymousIdRelation, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_contact_anonymous_id_relation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        relation: &ContactAnonymousIdRelation,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_contacts_anonymous_ids
            (contact_id, anonymous_id)
            VALUES ($1, $2)";

        match sqlx::query(QUERY)
            .bind(relation.contact_id)
            .bind(relation.anonymous_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.create_contact_anonymous_id_relation: Inserting relation: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
