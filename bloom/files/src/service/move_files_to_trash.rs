use super::{MoveFilesToTrashInput, Service};
use kernel::entities::User;

impl Service {
    pub async fn move_files_to_trash(
        &self,
        _actor: Option<User>,
        _input: MoveFilesToTrashInput,
    ) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
