use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_group<'c, C: db::Queryer<'c>>(&self, db: C, group: &entities::Group) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_groups SET
		updated_at = $1, path = $2, name = $3, description = $4, avatar_id = $5
		WHERE id = $6";

        match sqlx::query(QUERY)
            .bind(group.updated_at)
            .bind(&group.path)
            .bind(&group.name)
            .bind(&group.description)
            .bind(&group.avatar_id)
            .bind(group.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_group: updating group: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
