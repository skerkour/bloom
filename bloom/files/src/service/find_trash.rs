use super::Service;
use crate::entities::File;
use kernel::Actor;

impl Service {
    pub async fn find_trash(&self, _actor: Actor, _namespace: String) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
