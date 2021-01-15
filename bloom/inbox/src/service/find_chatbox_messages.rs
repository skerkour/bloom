use super::FindChatboxMessagesInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn find_chatbox_messages(
        &self,
        _actor: Actor,
        _input: FindChatboxMessagesInput,
    ) -> Result<Vec<Message>, kernel::Error> {
        todo!();
    }
}
