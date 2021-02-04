use super::SendNewsletterMessageInput;
use crate::{entities::NewsletterMessage, Error, Service};
use kernel::{consts::BillingPlan, Actor};

impl Service {
    pub async fn send_newsletter_message(
        &self,
        actor: Actor,
        input: SendNewsletterMessageInput,
    ) -> Result<NewsletterMessage, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;

        let (namespace, _) = self
            .kernel_service
            .find_namespace_and_membership(&self.db, actor.id, message.namespace_id)
            .await
            .map_err(|_| Error::NewsletterMessageNotFound)?;

        if !self.kernel_service.self_hosted() {
            if namespace.plan == BillingPlan::Free {
                return Err(Error::UpgradePlanToSendNewsletterMessage.into());
            }
        }

        let job = kernel::domain::messages::Message::InboxDispatchSendNewsletterMessage {
            message_id: message.id,
        };
        self.queue.push(job, None).await?;

        Ok(message)
    }
}
