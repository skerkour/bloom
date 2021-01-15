use super::SendMessageInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn send_message(&self, _actor: Actor, _input: SendMessageInput) -> Result<Message, kernel::Error> {
        todo!();
    }
}
