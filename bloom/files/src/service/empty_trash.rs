use super::{EmptyTrashInput, Service};
use kernel::entities::User;

impl Service {
    pub async fn empty_trash(&self, _actor: Option<User>, _input: EmptyTrashInput) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
