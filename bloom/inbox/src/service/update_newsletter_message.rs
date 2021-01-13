use super::UpdateNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn update_newsletter_message(
        &self,
        _actor: Actor,
        _input: UpdateNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        todo!();
    }
}
