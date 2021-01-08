use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn update_customer<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        customer: &entities::Customer,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE kernel_customers SET
            updated_at = $1, subscription_updated_at = $2, plan = $3, name = $4, email = $5,
            country = $6, country_code = $7, city = $8, postal_code = $9, address_line1 = $10,
            address_line2 = $11, state = $12, tax_id_type = $13, tax_id = $14, stripe_subscription_id = $15,
            stripe_product_id = $16, stripe_price_id = $17, stripe_tax_id = $18, stripe_default_payment_method_id = $19,
            namespace_id = $20
            WHERE id = $21";

        match sqlx::query(QUERY)
            .bind(customer.updated_at)
            .bind(customer.subscription_updated_at)
            .bind(customer.plan)
            .bind(&customer.name)
            .bind(&customer.email)
            .bind(&customer.country)
            .bind(&customer.country_code)
            .bind(&customer.city)
            .bind(&customer.postal_code)
            .bind(&customer.address_line1)
            .bind(&customer.address_line2)
            .bind(&customer.state)
            .bind(customer.tax_id_type)
            .bind(&customer.tax_id)
            .bind(&customer.stripe_subscription_id)
            .bind(&customer.stripe_product_id)
            .bind(&customer.stripe_price_id)
            .bind(&customer.stripe_tax_id)
            .bind(&customer.stripe_default_payment_method_id)
            .bind(customer.namespace_id)
            .bind(customer.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.update_customer: updating customer: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
