use super::Repository;
use crate::{entities::FilePath, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn get_file_path<'c, C: Queryer<'c>>(&self, db: C, file_id: Uuid) -> Result<Vec<FilePath>, Error> {
        const QUERY: &str = "WITH ancestors AS (
            WITH RECURSIVE tree AS (
                SELECT id, name, created_at, ARRAY[]::UUID[] AS ancestors
                FROM files WHERE parent_id IS NULL

                UNION ALL

                SELECT files.id, files.name, files.created_at, tree.ancestors || files.parent_id
                FROM files, tree
                WHERE files.parent_id = tree.id
            ) SELECT * FROM tree WHERE id = $1
        ) SELECT files.id, files.name FROM files, ancestors
        WHERE files.id = ANY(ancestors.ancestors) OR files.id = $1
        ORDER BY files.created_at;";

        match sqlx::query_as::<_, FilePath>(QUERY).bind(file_id).fetch_all(db).await {
            Err(err) => {
                error!("files.get_file_path: Finding path: {}", &err);
                Err(err.into())
            }
            Ok(files) => Ok(files),
        }
    }
}
