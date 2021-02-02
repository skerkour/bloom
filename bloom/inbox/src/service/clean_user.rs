use crate::Service;
use stdx::{
    sqlx::{Postgres, Transaction},
    uuid::Uuid,
};

impl Service {
    pub async fn clean_user<'c>(&self, tx: &mut Transaction<'c, Postgres>, user_id: Uuid) -> Result<(), kernel::Error> {
        self.repo.detach_messages_from_user(tx, user_id).await?;
        Ok(())
    }
}
