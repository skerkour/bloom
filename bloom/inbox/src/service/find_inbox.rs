use super::FindInboxInput;
use crate::{entities::Conversation, Service};
use kernel::Actor;

impl Service {
    pub async fn find_inbox(&self, _actor: Actor, _input: FindInboxInput) -> Result<Vec<Conversation>, kernel::Error> {
        todo!();
    }
}
