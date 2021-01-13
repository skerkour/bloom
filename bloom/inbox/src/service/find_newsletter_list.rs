use super::FindNewsletterListInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_list(
        &self,
        _actor: Actor,
        _input: FindNewsletterListInput,
    ) -> Result<NewsletterList, kernel::Error> {
        todo!();
    }
}
