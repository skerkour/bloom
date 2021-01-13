use super::FindNewsletterMessagesInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_messages(
        &self,
        _actor: Actor,
        _input: FindNewsletterMessagesInput,
    ) -> Result<Vec<NewsletterMessage>, kernel::Error> {
        todo!();
    }
}
