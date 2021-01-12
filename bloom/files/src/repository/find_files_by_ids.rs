use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_files_by_ids<'c, C: Queryer<'c>>(&self, db: C, file_ids: &[Uuid]) -> Result<Vec<File>, Error> {
        const QUERY: &str = "SELECT * FROM files
            WHERE id = ANY($1)";

        match sqlx::query_as::<_, File>(QUERY).bind(file_ids).fetch_all(db).await {
            Err(err) => {
                error!("files.find_files_by_ids: Finding files: {}", &err);
                Err(err.into())
            }
            Ok(files) => Ok(files),
        }
    }
}
