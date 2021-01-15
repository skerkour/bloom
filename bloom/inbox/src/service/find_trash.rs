use super::FindTrashInput;
use crate::{entities::Conversation, Service};
use kernel::Actor;

impl Service {
    pub async fn find_trash(&self, _actor: Actor, _input: FindTrashInput) -> Result<Vec<Conversation>, kernel::Error> {
        todo!();
    }
}
