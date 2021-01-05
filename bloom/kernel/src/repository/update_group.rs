use stdx::log::error;

use super::Repository;
use crate::{db, entities, errors::kernel::Error};

impl Repository {
    pub async fn update_group<'c, C: db::Queryer<'c>>(&self, db: C, group: &entities::Group) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_groups SET
		updated_at = $1, path = $2, name = $3, description = $4, avatar = $5, used_storage = $6, plan = $7
		WHERE id = $8";

        match sqlx::query(QUERY)
            .bind(group.updated_at)
            .bind(&group.path)
            .bind(&group.name)
            .bind(&group.description)
            .bind(&group.avatar)
            .bind(group.used_storage)
            .bind(group.plan)
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
