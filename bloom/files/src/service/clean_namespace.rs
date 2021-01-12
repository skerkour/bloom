use crate::Service;
use stdx::{
    sqlx::{Postgres, Transaction},
    uuid::Uuid,
};

impl Service {
    pub async fn clean_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        namespace_id: Uuid,
    ) -> Result<(), kernel::Error> {
        self.repo.detach_all_files_from_namespace(tx, namespace_id).await?;
        Ok(())
    }
}
