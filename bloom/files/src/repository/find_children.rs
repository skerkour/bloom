use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_children<'c, C: Queryer<'c>>(&self, db: C, parent_id: Uuid) -> Result<Vec<File>, Error> {
        const QUERY: &str = "SELECT * FROM files WHERE parent_id = $1 AND trashed_at IS NULL";

        match sqlx::query_as::<_, File>(QUERY).bind(parent_id).fetch_all(db).await {
            Err(err) => {
                error!("files.find_children: Finding files: {}", &err);
                Err(err.into())
            }
            Ok(files) => Ok(files),
        }
    }
}
