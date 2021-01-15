use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_file<'c, C: Queryer<'c>>(&self, db: C, file: &File) -> Result<(), Error> {
        const QUERY: &str = "UPDATE files SET
            updated_at = $1, name = $2, size = $3, type = $4, explicitly_trashed = $5, trashed_at = $6, parent_id = $7, namespace_id = $8
            WHERE id = $9";

        match sqlx::query(QUERY)
            .bind(file.updated_at)
            .bind(&file.name)
            .bind(file.size)
            .bind(&file.r#type)
            .bind(file.explicitly_trashed)
            .bind(file.trashed_at)
            .bind(file.parent_id)
            .bind(file.namespace_id)
            .bind(file.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.update_file: Updating file: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
