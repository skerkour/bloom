use super::{CompleteFileUploadInput, Service};
use crate::entities::File;
use kernel::Actor;

impl Service {
    pub async fn complete_file_upload(
        &self,
        _actor: Actor,
        _input: CompleteFileUploadInput,
    ) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
