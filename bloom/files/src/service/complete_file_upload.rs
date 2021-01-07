use kernel::entities::User;
use super::{CompleteFileUploadInput, Service};
use crate::entities::File;

impl Service {
    pub async fn complete_file_upload(
        &self,
        _actor: Option<User>,
        _input: CompleteFileUploadInput,
    ) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
