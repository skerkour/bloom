use super::FindContactsInput;
use crate::{entities::Contact, Service};
use kernel::Actor;

impl Service {
    pub async fn find_contacts(&self, _actor: Actor, _input: FindContactsInput) -> Result<Vec<Contact>, kernel::Error> {
        todo!();
    }
}
