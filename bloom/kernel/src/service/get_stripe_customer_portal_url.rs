use crate::{consts::NamespaceType, errors::kernel::Error, Actor, Service};
use stdx::{stripe, uuid::Uuid};

impl Service {
    pub async fn get_stripe_customer_portal_url(
        &self,
        actor: Actor,
        namespace_id: Uuid,
    ) -> Result<String, crate::Error> {
        let actor = self.current_user(actor)?;

        self.check_namespace_membership(&self.db, &actor, namespace_id).await?;

        if self.self_hosted() {
            return Err(Error::BillingCantBeAccessedWhenSelfHosting.into());
        }

        let namespace = self.repo.find_namespace_by_id(&self.db, namespace_id).await?;
        let customer = self.repo.find_customer_by_namespace_id(&self.db, namespace_id).await?;

        let return_url = match namespace.r#type {
            NamespaceType::User => format!("{}/preferences/billing/sync", &self.config.base_url),
            NamespaceType::Group => format!("{}/groups/{}/billing/sync", &self.config.base_url, &namespace.path),
        };
        // returnURL := fmt.Sprintf("%s/groups/%s/-/billing/sync", service.config.BaseURL, namespace.Namespace.Path)

        let portal_params = stripe::model::BillingPortalSessionParams {
            customer: customer.stripe_customer_id,
            return_url,
        };
        // unwrap is safe as if we are here we are not self-hosting
        let stripe_customer_portal_session = self
            .stripe_client
            .as_ref()
            .unwrap()
            .new_billing_portal_session(portal_params)
            .await?;

        Ok(stripe_customer_portal_session.url)
    }
}
