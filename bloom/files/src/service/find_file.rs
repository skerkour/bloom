use super::{FindFileInput, Service};
use crate::entities::File;
use kernel::Actor;

impl Service {
    pub async fn find_file(&self, _actor: Actor, _input: FindFileInput) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
