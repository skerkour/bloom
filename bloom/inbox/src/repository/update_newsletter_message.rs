use super::Repository;
use crate::{entities::NewsletterMessage, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_newsletter_message<'c, C: Queryer<'c>>(
        &self,
        db: C,
        message: &NewsletterMessage,
    ) -> Result<(), Error> {
        todo!();
    }
}
