use super::{MoveFilesInput, Service};
use kernel::Actor;

impl Service {
    pub async fn move_files(&self, _actor: Actor, _input: MoveFilesInput) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
