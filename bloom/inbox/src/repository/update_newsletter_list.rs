use super::Repository;
use crate::{entities::NewsletterList, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_newsletter_list<'c, C: Queryer<'c>>(&self, db: C, list: &NewsletterList) -> Result<(), Error> {
        const QUERY: &str = "UPDATE newsletter_lists SET
            updated_at = $1, name = $2, description = $3
            WHERE id = $4";

        match sqlx::query(QUERY)
            .bind(list.updated_at)
            .bind(&list.name)
            .bind(&list.description)
            .bind(list.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.update_newsletter_list: Updating list: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
