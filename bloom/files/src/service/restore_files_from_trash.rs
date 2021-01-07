use super::{RestoreFilesFromTrashInput, Service};
use kernel::entities::User;

impl Service {
    pub async fn restore_files_from_trash(
        &self,
        _actor: Option<User>,
        _input: RestoreFilesFromTrashInput,
    ) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
