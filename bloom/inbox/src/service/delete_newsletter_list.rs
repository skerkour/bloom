use super::DeleteNewsletterListInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_newsletter_list(
        &self,
        _actor: Actor,
        _input: DeleteNewsletterListInput,
    ) -> Result<(), kernel::Error> {
        todo!();
    }
}
