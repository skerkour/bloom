use super::SendChatboxMessageInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn send_chatbox_message(
        &self,
        _actor: Actor,
        _input: SendChatboxMessageInput,
    ) -> Result<Message, kernel::Error> {
        todo!();
    }
}
