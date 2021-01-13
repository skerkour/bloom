use super::SendTestNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn send_test_newsletter_message(
        &self,
        _actor: Actor,
        _input: SendTestNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        todo!();
    }
}
