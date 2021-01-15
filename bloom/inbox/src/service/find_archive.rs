use super::FindArchiveInput;
use crate::{entities::Conversation, Service};
use kernel::Actor;

impl Service {
    pub async fn find_archive(
        &self,
        _actor: Actor,
        _input: FindArchiveInput,
    ) -> Result<Vec<Conversation>, kernel::Error> {
        todo!();
    }
}
