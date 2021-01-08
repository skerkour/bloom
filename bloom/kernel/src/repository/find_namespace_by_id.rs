use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_namespace_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<entities::Namespace, Error> {
        const QUERY: &str = "SELECT * FROM kernel_namespaces WHERE id = $1";

        match sqlx::query_as::<_, entities::Namespace>(QUERY)
            .bind(namespace_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_namespace_by_id: finding namespace: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::NamespaceNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
