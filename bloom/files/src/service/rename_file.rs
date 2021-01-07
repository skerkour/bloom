use super::{RenameFileInput, Service};
use crate::entities::File;
use kernel::entities::User;

impl Service {
    pub async fn rename_file(&self, _actor: Option<User>, _input: RenameFileInput) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
