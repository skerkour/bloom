use super::Service;
use crate::entities::File;
use kernel::entities::User;

impl Service {
    pub async fn find_trash(&self, _actor: Option<User>, _namespace: String) -> Result<File, kernel::Error> {
        unimplemented!();
    }
}
