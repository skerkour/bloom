use super::{CreateFolderInput, Service};
use crate::entities::File;
use kernel::entities::User;

impl Service {
    pub async fn create_folder(&self, _actor: Option<User>, _input: CreateFolderInput) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
