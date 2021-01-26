use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_namespace<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        namespace: &entities::Namespace,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_namespaces SET
		updated_at = $1, path = $2, type = $3, parent_id = $4, used_storage = $5, plan = $6
		WHERE id = $7";

        match sqlx::query(QUERY)
            .bind(namespace.updated_at)
            .bind(&namespace.path)
            .bind(namespace.r#type)
            .bind(namespace.parent_id)
            .bind(namespace.used_storage)
            .bind(namespace.plan)
            .bind(namespace.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_namespace: updating namespace: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
