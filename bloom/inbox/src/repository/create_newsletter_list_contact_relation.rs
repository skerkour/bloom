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
        todo!();
    }
}
