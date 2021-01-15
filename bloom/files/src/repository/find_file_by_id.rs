use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_file_by_id<'c, C: Queryer<'c>>(&self, db: C, file_id: Uuid) -> Result<entities::File, Error> {
        const QUERY: &str = "SELECT * FROM files
            WHERE id = $1";

        match sqlx::query_as::<_, entities::File>(QUERY)
            .bind(file_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("files.find_file_by_id: finding file: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::FileNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
