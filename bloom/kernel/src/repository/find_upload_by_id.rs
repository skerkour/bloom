use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_upload_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        upload_id: Uuid,
    ) -> Result<entities::Upload, Error> {
        const QUERY: &str = "SELECT * FROM kernel_uploads WHERE id = $1";

        match sqlx::query_as::<_, entities::Upload>(QUERY)
            .bind(upload_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_upload_by_id: finding uploads: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::UploadNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
