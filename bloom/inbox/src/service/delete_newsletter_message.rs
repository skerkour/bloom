use super::DeleteNewsletterMessageInput;
use crate::Service;
use kernel::Actor;

impl Service {
    pub async fn delete_newsletter_message(
        &self,
        _actor: Actor,
        _input: DeleteNewsletterMessageInput,
    ) -> Result<(), kernel::Error> {
        todo!();
    }
}
