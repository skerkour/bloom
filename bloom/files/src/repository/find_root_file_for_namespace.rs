use super::Repository;
use crate::{consts, entities, Error};
use kernel::db::Queryer;
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_root_file_for_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<entities::File, Error> {
        const QUERY: &str = "SELECT * FROM files
            WHERE namespace_id = $1 AND name = $2";

        match sqlx::query_as::<_, entities::File>(QUERY)
            .bind(namespace_id)
            .bind(consts::ROOT_FILE_NAME)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("files.find_root_file_for_namespace: finding file: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::FileNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
