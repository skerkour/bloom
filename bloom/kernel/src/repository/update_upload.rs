use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_upload<'c, C: db::Queryer<'c>>(&self, db: C, upload: &entities::Upload) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_uploads SET
		updated_at = $1, completed = $2
		WHERE id = $3";

        match sqlx::query(QUERY)
            .bind(upload.updated_at)
            .bind(upload.completed)
            .bind(upload.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_upload: updating upload: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
