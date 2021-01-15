use super::FindTrashInput;
use crate::{entities::Message, Service};
use kernel::Actor;

impl Service {
    pub async fn find_trash(&self, _actor: Actor, _input: FindTrashInput) -> Result<Vec<Message>, kernel::Error> {
        todo!();
    }
}
