use super::FindNewsletterListsInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;

impl Service {
    pub async fn find_newsletter_lists(
        &self,
        _actor: Actor,
        _input: FindNewsletterListsInput,
    ) -> Result<Vec<NewsletterList>, kernel::Error> {
        todo!();
    }
}
