use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{log::error, uuid::Uuid};

impl Repository {
    pub async fn find_customer_by_namespace_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<entities::Customer, Error> {
        const QUERY: &str = "SELECT * FROM kernel_customers WHERE namespace_id = $1";

        match sqlx::query_as::<_, entities::Customer>(QUERY)
            .bind(namespace_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_customer_by_namespace_id: finding customer: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::CustomerNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
