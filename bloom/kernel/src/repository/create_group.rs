use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_group<'c, C: db::Queryer<'c>>(&self, db: C, group: &entities::Group) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_groups
            (id, created_at, updated_at, path, name, description, avatar_id, namespace_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

        match sqlx::query(QUERY)
            .bind(group.id)
            .bind(group.created_at)
            .bind(group.updated_at)
            .bind(&group.path)
            .bind(&group.name)
            .bind(&group.description)
            .bind(&group.avatar_id)
            .bind(group.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_group: Inserting group: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
