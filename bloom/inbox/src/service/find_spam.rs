use super::FindSpamInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn find_spam(&self, _actor: Actor, _input: FindSpamInput) -> Result<Vec<Message>, kernel::Error> {
        todo!();
    }
}
