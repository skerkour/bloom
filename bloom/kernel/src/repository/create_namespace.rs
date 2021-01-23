use super::Repository;
use crate::{entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{
    log::error,
    sqlx::{Postgres, Transaction},
};

impl Repository {
    pub async fn create_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        namespace: &entities::Namespace,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_namespaces
            (id, created_at, updated_at, path, type, parent_id, used_storage, plan)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

        match sqlx::query(QUERY)
            .bind(namespace.id)
            .bind(namespace.created_at)
            .bind(namespace.updated_at)
            .bind(&namespace.path)
            .bind(namespace.r#type)
            .bind(namespace.parent_id)
            .bind(namespace.used_storage)
            .bind(namespace.plan)
            .execute(tx)
            .await
        {
            Err(err) => {
                error!("kernel.create_namespace: Inserting namespace: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
