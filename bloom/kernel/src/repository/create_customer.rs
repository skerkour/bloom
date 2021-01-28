use super::Repository;
use crate::{db, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;

impl Repository {
    pub async fn create_customer<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        customer: &entities::Customer,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO kernel_customers
        (id, created_at, updated_at, subscription_updated_at, plan, name, email, country, country_code,
            city, postal_code, address_line1, address_line2, state, tax_id_type, tax_id, stripe_customer_id,
            stripe_subscription_id, stripe_product_id, stripe_price_id, stripe_tax_id, stripe_default_payment_method_id,
            namespace_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)";

        match sqlx::query(QUERY)
            .bind(customer.id)
            .bind(customer.created_at)
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
            .bind(&customer.stripe_customer_id)
            .bind(&customer.stripe_subscription_id)
            .bind(&customer.stripe_product_id)
            .bind(&customer.stripe_price_id)
            .bind(&customer.stripe_tax_id)
            .bind(&customer.stripe_default_payment_method_id)
            .bind(customer.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("kernel.create_customer: Inserting customer: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
