use super::UnsubscribeFromListInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn unsubscribe_from_list(
        &self,
        _actor: Actor,
        _input: UnsubscribeFromListInput,
    ) -> Result<(), kernel::Error> {
        todo!();
    }
}
