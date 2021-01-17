use super::SubscribeToListInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn subscribe_to_list(&self, _actor: Actor, _input: SubscribeToListInput) -> Result<(), kernel::Error> {
        todo!();
    }
}
