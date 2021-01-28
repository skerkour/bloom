use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn find_customer_by_stripe_customer_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        stripe_customer_id: &str,
    ) -> Result<entities::Customer, Error> {
        const QUERY: &str = "SELECT * FROM kernel_customers WHERE stripe_customer_id = $1";

        match sqlx::query_as::<_, entities::Customer>(QUERY)
            .bind(stripe_customer_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_customer_by_stripe_customer_id: finding customer: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::CustomerNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
