use super::Repository;
use crate::{entities::NewsletterList, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_newsletter_list<'c, C: Queryer<'c>>(&self, db: C, list: &NewsletterList) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO newsletter_lists
            (id, created_at, updated_at, name, description, namespace_id)
            VALUES ($1, $2, $3, $4, $5, $6)";

        match sqlx::query(QUERY)
            .bind(list.id)
            .bind(list.created_at)
            .bind(list.updated_at)
            .bind(&list.name)
            .bind(&list.description)
            .bind(list.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("inbox.create_newsletter_list: Inserting list: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
