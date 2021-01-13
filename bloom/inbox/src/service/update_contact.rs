use super::UpdateContactInput;
use crate::{entities::Contact, Service};
use kernel::Actor;

impl Service {
    pub async fn update_contact(&self, _actor: Actor, _input: UpdateContactInput) -> Result<Contact, kernel::Error> {
        todo!();
    }
}
