use crate::{consts::BillingPlan, errors::kernel::Error, Actor, Service};
use stdx::{chrono::Utc, stripe, uuid::Uuid};

impl Service {
    pub async fn sync_customer_with_stripe(
        &self,
        actor: Option<Actor>,
        namespace_id: Option<Uuid>,
        stripe_customer_id: Option<String>,
    ) -> Result<(), crate::Error> {
        let mut customer = if let Some(namespace_id) = namespace_id {
            self.repo.find_customer_by_namespace_id(&self.db, namespace_id).await?
        } else if let Some(stripe_customer_id) = stripe_customer_id {
            self.repo
                .find_customer_by_stripe_customer_id(&self.db, &stripe_customer_id)
                .await?
        } else {
            return Err(Error::PermissionDenied.into());
        };

        if self.self_hosted() {
            return Err(Error::BillingCantBeAccessedWhenSelfHosting.into());
        }

        if customer.namespace_id.is_none() {
            // early return beacuse the customer is no longer linked to an active namespace
            return Ok(());
        }

        if let Some(actor) = actor {
            let actor = self.current_user(actor)?;
            if !actor.is_admin {
                self.check_namespace_membership(&self.db, &actor, customer.namespace_id.unwrap())
                    .await?;
            }
        }

        let mut namespace = self
            .repo
            .find_namespace_by_id(&self.db, customer.namespace_id.unwrap())
            .await?;

        // unwrap is safe as if we are here we are not self-hosting
        let stripe_data = self.config.stripe.as_ref().unwrap().data.clone();
        let now = Utc::now();

        customer.updated_at = now;
        namespace.updated_at = now;

        // fetch stripe customer
        let expands = vec![
            "subscriptions".into(),
            "sources".into(),
            "default_source".into(),
            "subscriptions.data.plan".into(),
            "subscriptions.data.plan.product".into(),
            "invoice_settings.default_payment_method".into(),
        ];
        let stripe_customer_params = stripe::model::CustomerParams {
            expand: Some(expands),
            ..Default::default()
        };
        let stripe_customer = self
            .stripe_client
            .as_ref()
            .unwrap()
            .get_customer(&customer.stripe_customer_id, stripe_customer_params)
            .await?;

        if stripe_customer.subscriptions.is_some() && stripe_customer.subscriptions.clone().unwrap().data.len() == 1 {
            let subscription = stripe_customer.subscriptions.unwrap().data[0].clone();
            customer.stripe_subscription_id = Some(subscription.id);
            customer.stripe_price_id = Some(subscription.plan.id);
            customer.stripe_product_id = Some(subscription.plan.product.id);

            if customer.stripe_product_id == Some(stripe_data.products.starter) {
                customer.plan = BillingPlan::Starter;
            } else if customer.stripe_product_id == Some(stripe_data.products.pro) {
                customer.plan = BillingPlan::Pro;
            // } else if customer.stripe_product_id == Some(stripe_data.products.ultra) {
            //     customer.plan = BillingPlan::Ultra;
            } else {
                customer.plan = BillingPlan::Free;
            }
        } else {
            customer.plan = BillingPlan::Free;
            customer.stripe_subscription_id = None;
            customer.stripe_price_id = None;
            customer.stripe_product_id = None;
        }

        if stripe_customer.invoice_settings.is_some()
            && stripe_customer
                .invoice_settings
                .clone()
                .unwrap()
                .default_payment_method
                .is_some()
        {
            customer.stripe_default_payment_method_id = Some(
                stripe_customer
                    .invoice_settings
                    .unwrap()
                    .default_payment_method
                    .unwrap()
                    .id
                    .clone(),
            );
        } else {
            customer.stripe_default_payment_method_id = None;
        }

        namespace.plan = customer.plan;

        let mut tx = self.db.begin().await?;
        self.repo.update_namespace(&mut tx, &namespace).await?;
        self.repo.update_customer(&mut tx, &customer).await?;

        tx.commit().await?;

        Ok(())
    }
}
