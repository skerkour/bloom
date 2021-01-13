use super::SendNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Service};
use kernel::Actor;

impl Service {
    pub async fn send_newsletter_message(&self, _actor: Actor, _input: SendNewsletterMessageInput) -> Result<NewsletterMessage, kernel::Error> {
        todo!();
    }
}
