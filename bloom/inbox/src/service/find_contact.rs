use super::FindContactInput;
use crate::{entities::Contact, Service};
use kernel::Actor;

impl Service {
    pub async fn find_contact(&self, _actor: Actor, _input: FindContactInput) -> Result<Contact, kernel::Error> {
        todo!();
    }
}
