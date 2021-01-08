use super::{MoveFilesToTrashInput, Service};
use kernel::Actor;

impl Service {
    pub async fn move_files_to_trash(&self, _actor: Actor, _input: MoveFilesToTrashInput) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
