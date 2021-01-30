use super::GetStripeCheckoutSessionInput;
use crate::{consts::BillingPlan, errors::kernel::Error, Actor, Service};
use stdx::stripe;

impl Service {
    pub async fn get_stripe_checkout_session(
        &self,
        actor: Actor,
        input: GetStripeCheckoutSessionInput,
    ) -> Result<String, crate::Error> {
        // check namespace
        let actor = self.current_user(actor)?;

        self.check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let namespace = self.repo.find_namespace_by_id(&self.db, input.namespace_id).await?;
        let customer = self
            .repo
            .find_customer_by_namespace_id(&self.db, input.namespace_id)
            .await?;

        if customer.stripe_subscription_id.is_some() {
            return Err(Error::PermissionDenied.into());
        }

        if input.plan == BillingPlan::Free {
            return Err(Error::PermissionDenied.into());
        }

        let stripe_data = self.config.stripe.data.clone();
        let mut stripe_tax_rates: Vec<String> = Vec::new();

        let stripe_price = match input.plan {
            BillingPlan::Starter => stripe_data.prices.starter,
            BillingPlan::Pro => stripe_data.prices.pro,
            // BillingPlan::Ultra => stripe_data.prices.ultra,
            _ => return Err(Error::PermissionDenied.into()),
        };

        if customer.tax_id.is_none() {
            if let Some(stripe_tax) = stripe_data.taxes.get(&customer.country_code) {
                stripe_tax_rates.push(stripe_tax.clone())
            }
        }

        // if customer.TaxID == nil {
        // 	stripeTaxID, stripeTaxIDFound := stripeData.Taxes[customer.CountryCode]
        // 	if stripeTaxIDFound {
        // 		stripeSubscriptionData = &stripe.CheckoutSessionSubscriptionDataParams{
        // 			DefaultTaxRates: stripe.StringSlice([]string{
        // 				stripeTaxID,
        // 			}),
        // 		}
        // 	}
        // }

        let cancel_url = format!("{}/groups/{}/billing", &self.config.base_url, &namespace.path);
        let success_url = format!("{}/sync", &cancel_url);
        // cancelURL := fmt.Sprintf("%s/groups/%s/-/billing", service.config.BaseURL, namespace.Namespace.Path)
        // successURL := fmt.Sprintf("%s/sync", cancelURL)

        let params = stripe::model::CheckoutSessionParams {
            customer: customer.stripe_customer_id,
            payment_method_type: vec!["card".to_string()],
            line_items: vec![stripe::model::CheckoutSessionLineItemParams {
                price: stripe_price,
                quantity: 1,
            }],
            mode: String::from("subscription"),
            success_url,
            cancel_url,
            subscription_data: Some(stripe::model::CheckoutSessionSubscriptionDataParams {
                default_tax_rates: stripe_tax_rates,
            }),
        };
        let checkout_session = self.stripe_client.new_checkout_session(params).await?;

        Ok(checkout_session.id)
    }
}
