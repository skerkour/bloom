use crate::{errors::kernel::Error, Actor, Service};

impl Service {
    pub fn get_stripe_public_key(&self, actor: Actor) -> Result<String, crate::Error> {
        let _ = self.current_user(actor)?;

        if self.self_hosted() {
            return Err(Error::BillingCantBeAccessedWhenSelfHosting.into());
        }

        Ok(self.config.stripe.as_ref().unwrap().public_key.clone())
    }
}
