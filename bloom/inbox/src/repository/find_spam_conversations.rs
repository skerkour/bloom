use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_spam_conversations<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<entities::Conversation>, Error> {
        todo!();
    }
}
