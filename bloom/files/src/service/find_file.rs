use super::{FindFileInput, Service};
use crate::entities::File;
use kernel::entities::User;

impl Service {
    pub async fn find_file(&self, _actor: Option<User>, _input: FindFileInput) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
