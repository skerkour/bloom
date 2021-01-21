use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_upload<'c, C: Queryer<'c>>(&self, db: C, upload_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_uploads WHERE id = $1";

        match sqlx::query(QUERY).bind(upload_id).execute(db).await {
            Err(err) => {
                error!("kernel.delete_upload: Deleting upload: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
