use super::CreateContactInput;
use crate::{entities::Contact, Service};
use kernel::Actor;

impl Service {
    pub async fn create_contact(&self, _actor: Actor, _input: CreateContactInput) -> Result<Contact, kernel::Error> {
        todo!();
    }
}
