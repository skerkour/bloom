use super::{MoveFilesInput, Service};
use kernel::entities::User;

impl Service {
    pub async fn move_files(&self, _actor: Option<User>, _input: MoveFilesInput) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
