use super::{RestoreFilesFromTrashInput, Service};
use kernel::Actor;

impl Service {
    pub async fn restore_files_from_trash(
        &self,
        _actor: Actor,
        _input: RestoreFilesFromTrashInput,
    ) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
