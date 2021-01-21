use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_upload<'c, C: db::Queryer<'c>>(&self, db: C, upload: &entities::Upload) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_uploads
            (id, created_at, updated_at, size, completed, namespace_id)
            VALUES ($1, $2, $3, $4, $5, $6)";

        match sqlx::query(QUERY)
            .bind(upload.id)
            .bind(upload.created_at)
            .bind(upload.updated_at)
            .bind(upload.size)
            .bind(upload.completed)
            .bind(upload.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_upload: Inserting upload: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
