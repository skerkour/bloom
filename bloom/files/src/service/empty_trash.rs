use super::{EmptyTrashInput, Service};
use kernel::Actor;

impl Service {
    pub async fn empty_trash(&self, _actor: Actor, _input: EmptyTrashInput) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
