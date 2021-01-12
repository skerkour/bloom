use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_file_by_parent_and_name<'c, C: Queryer<'c>>(
        &self,
        db: C,
        parent_id: Uuid,
        name: &str,
    ) -> Result<entities::File, Error> {
        const QUERY: &str = "SELECT * FROM files
            WHERE parent_id = $1 AND name = $2 AND trashed_at IS NULL";

        match sqlx::query_as::<_, entities::File>(QUERY)
            .bind(parent_id)
            .bind(&name)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("files.find_file_by_parent_and_name: finding file: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::FileNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
