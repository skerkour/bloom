use super::FindNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_message(
        &self,
        _actor: Actor,
        _input: FindNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        todo!();
    }
}
