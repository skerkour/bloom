use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_file<'c, C: Queryer<'c>>(&self, db: C, file: &File) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO files
        (id, created_at, updated_at, name, size, type, explicitly_trashed, trashed_at, namespace_id, parent_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";

        match sqlx::query(QUERY)
            .bind(file.id)
            .bind(file.created_at)
            .bind(file.updated_at)
            .bind(&file.name)
            .bind(file.size)
            .bind(&file.r#type)
            .bind(file.explicitly_trashed)
            .bind(file.trashed_at)
            .bind(file.namespace_id)
            .bind(file.parent_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.create_file: Inserting file: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
