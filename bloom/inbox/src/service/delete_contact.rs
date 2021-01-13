use super::DeleteContactInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_contact(&self, _actor: Actor, _input: DeleteContactInput) -> Result<(), kernel::Error> {
        todo!();
    }
}
