use super::Repository;
use crate::{entities::Conversation, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_conversation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        conversation: &Conversation,
    ) -> Result<(), Error> {
        todo!();
    }
}
