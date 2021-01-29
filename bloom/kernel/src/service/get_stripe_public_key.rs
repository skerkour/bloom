use crate::{Actor, Error, Service};

impl Service {
    pub fn get_stripe_public_key(&self, actor: Actor) -> Result<String, Error> {
        let _ = self.current_user(actor)?;
        Ok(self.config.stripe.public_key.clone())
    }
}
