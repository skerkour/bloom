use super::DeleteEventInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_event(&self, _actor: Actor, _input: DeleteEventInput) -> Result<(), kernel::Error> {
        todo!();
    }
}
