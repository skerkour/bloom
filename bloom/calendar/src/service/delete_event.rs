use super::DeleteEventInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_event(&self, actor: Actor, input: DeleteEventInput) -> Result<(), kernel::Error> {
        todo!();
    }
}
