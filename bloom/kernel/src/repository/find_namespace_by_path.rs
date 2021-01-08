use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_namespace_by_path<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        path: &str,
    ) -> Result<entities::Namespace, Error> {
        const QUERY: &str = "SELECT * FROM kernel_namespaces WHERE path = $1";

        match sqlx::query_as::<_, entities::Namespace>(QUERY)
            .bind(path)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_namespace_by_path: finding namespace: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::NamespaceNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
