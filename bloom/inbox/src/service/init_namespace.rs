use super::InitNamespaceInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn init_namespace(&self, _actor: Actor, _input: InitNamespaceInput) -> Result<(), kernel::Error> {
        todo!();
    }
}
