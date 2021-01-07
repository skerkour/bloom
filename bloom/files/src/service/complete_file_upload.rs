use kernel::entities::User;

use super::{CompleteFileUpload, Service};
use crate::entities::File;

impl Service {
    pub async fn complete_file_upload(
        &self,
        _actor: Option<User>,
        _input: CompleteFileUpload,
    ) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
