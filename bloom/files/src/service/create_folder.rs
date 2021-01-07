use kernel::entities::User;
use super::{CreateFolderInput, Service};
use crate::entities::File;

impl Service {
    pub async fn create_folder(
        &self,
        _actor: Option<User>,
        _input: CreateFolderInput,
    ) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
