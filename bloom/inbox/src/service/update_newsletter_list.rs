use super::UpdateNewsletterListInput;
use crate::{entities::NewsletterList, Service};
use kernel::Actor;

impl Service {
    pub async fn update_newsletter_list(
        &self,
        _actor: Actor,
        _input: UpdateNewsletterListInput,
    ) -> Result<NewsletterList, kernel::Error> {
        todo!();
    }
}
