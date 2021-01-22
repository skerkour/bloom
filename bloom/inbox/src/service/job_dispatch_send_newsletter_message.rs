use super::Service;
use kernel::domain::messages::Message;
use std::collections::HashMap;
use stdx::{chrono::Utc, log::error, mail, uuid::Uuid};

impl Service {
    pub async fn job_dispatch_send_newsletter_message(&self, message_id: Uuid) -> Result<(), kernel::Error> {
        let mut message = self.repo.find_newsletter_message_by_id(&self.db, message_id).await?;

        let list = self.repo.find_newsletter_list_by_id(&self.db, message.list_id).await?;

        let contacts = self.repo.find_contacts_for_list(&self.db, list.id).await?;
        let subscriptions = self.repo.find_subscriptions_for_list(&self.db, list.id).await?;

        // this server side join is ugly... but fetching data from JOIN is sqlx is not easy... soo
        let subscriptions: HashMap<Uuid, Uuid> =
            subscriptions.into_iter().map(|sub| (sub.contact_id, sub.id)).collect();

        let now = Utc::now();
        message.updated_at = now;
        message.last_sent_at = Some(now);
        self.repo.update_newsletter_message(&self.db, &message).await?;

        // TODO: correct email of the sender
        let from = self.kernel_service.config().mail.notify_address.clone();

        for contact in contacts.into_iter().filter(|c| !c.email.is_empty()) {
            let to = mail::Address {
                name: contact.name,
                address: contact.email,
            };
            let subscription_id = subscriptions[&contact.id];
            let job = Message::InboxSendNewsletterMessage {
                message_id,
                from: from.clone(),
                to,
                subscription_id: Some(subscription_id),
            };
            match self.queue.push(job, None).await {
                Err(err) => {
                    error!(
                        "kernel.job_dispatch_send_newsletter_message: queueing message: {}",
                        &err
                    );
                }
                Ok(_) => {}
            };
        }

        Ok(())
    }
}
