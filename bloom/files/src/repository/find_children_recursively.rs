use super::Repository;
use crate::{entities::File, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_children_recursively<'c, C: Queryer<'c>>(
        &self,
        db: C,
        file_id: Uuid,
    ) -> Result<Vec<File>, Error> {
        const QUERY: &str = "WITH RECURSIVE tree AS (
            SELECT id, created_at, updated_at, name, size, type, explicitly_trashed, trashed_at, namespace_id,
                parent_id, ARRAY[]::UUID[] AS ancestors
            FROM files WHERE parent_id IS NULL

            UNION ALL

            SELECT files.id, files.created_at, files.updated_at,
            files.name, files.size, files.type,
            files.explicitly_trashed, files.trashed_at, files.namespace_id,
            files.parent_id, tree.ancestors || files.parent_id
            FROM files, tree
            WHERE files.parent_id = tree.id
          ) SELECT id, created_at, updated_at, name, size, type, explicitly_trashed, trashed_at, namespace_id, parent_id
          FROM tree WHERE $1 = ANY(tree.ancestors);";

        match sqlx::query_as::<_, File>(QUERY).bind(file_id).fetch_all(db).await {
            Err(err) => {
                error!("files.find_children_recursively: Finding files: {}", &err);
                Err(err.into())
            }
            Ok(files) => Ok(files),
        }
    }
}
