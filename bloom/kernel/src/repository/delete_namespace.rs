use super::Repository;
use crate::{db::Queryer, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn delete_namespace<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM kernel_namespaces WHERE id = $1";

        match sqlx::query(QUERY).bind(namespace_id).execute(db).await {
            Err(err) => {
                error!("kernel.delete_namespace: Deleting namespace: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
