use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_explicitly_trashed_files<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<Vec<File>, Error> {
        const QUERY: &str = "SELECT * FROM files
            WHERE namespace_id = $1 AND trashed_at IS NOT NULL AND explicitly_trashed = true";

        match sqlx::query_as::<_, File>(QUERY).bind(namespace_id).fetch_all(db).await {
            Err(err) => {
                error!("files.find_explicitly_trashed_files: Finding files: {}", &err);
                Err(err.into())
            }
            Ok(files) => Ok(files),
        }
    }
}
