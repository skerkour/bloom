use kernel::entities::User;
use super::{EmptyTrashInput, Service};

impl Service {
    pub async fn empty_trash(
        &self,
        _actor: Option<User>,
        _input: EmptyTrashInput,
    ) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
