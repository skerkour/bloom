use super::CreateNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn create_newsletter_message(
        &self,
        _actor: Actor,
        _input: CreateNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        todo!();
    }
}
