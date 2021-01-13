use super::CreateNewsletterListInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;

impl Service {
    pub async fn create_newsletter_list(
        &self,
        _actor: Actor,
        _input: CreateNewsletterListInput,
    ) -> Result<NewsletterList, kernel::Error> {
        todo!();
    }
}
