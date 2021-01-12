use super::Repository;
use crate::Error;
use kernel::db::Queryer;
use stdx::{chrono::Utc, log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn detach_all_trashed_files_from_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE files SET
        updated_at = $1, parent_id = NULL, namespace_id = NULL
        WHERE namespace_id = $2 AND trashed_at IS NOT NULL";

        let now = Utc::now();

        match sqlx::query(QUERY).bind(now).bind(namespace_id).execute(db).await {
            Err(err) => {
                error!("files.detach_all_trashed_files_from_namespace: Updating file: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
