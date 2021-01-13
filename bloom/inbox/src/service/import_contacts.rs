use super::ImportContactsInput;
use crate::{entities::Contact, Service};
use kernel::Actor;

impl Service {
    pub async fn import_contacts(
        &self,
        _actor: Actor,
        _input: ImportContactsInput,
    ) -> Result<Vec<Contact>, kernel::Error> {
        todo!();
    }
}
