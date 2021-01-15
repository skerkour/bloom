use super::Repository;
use crate::{entities::NewsletterList, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_newsletter_list<'c, C: Queryer<'c>>(&self, db: C, list: &NewsletterList) -> Result<(), Error> {
        todo!();
    }
}
