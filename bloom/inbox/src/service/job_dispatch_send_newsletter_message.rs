use super::Service;
use kernel::{consts::NamespaceType, domain::messages::Message};
use std::collections::HashMap;
use stdx::{
    chrono::{Duration, Utc},
    log::error,
    mail,
    uuid::Uuid,
};

impl Service {
    pub async fn job_dispatch_send_newsletter_message(&self, message_id: Uuid) -> Result<(), kernel::Error> {
        let mut message = self.repo.find_newsletter_message_by_id(&self.db, message_id).await?;
        let namespace = self
            .kernel_service
            .find_namespace(&self.db, message.namespace_id)
            .await?;

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

        let name = match namespace.r#type {
            NamespaceType::User => {
                let user = self
                    .kernel_service
                    .find_user_by_namespace_id_unauthenticated(&self.db, namespace.id)
                    .await?;
                user.name
            }
            NamespaceType::Group => {
                let group = self
                    .kernel_service
                    .find_group_by_namespace_id_unauthenticated(&self.db, namespace.id)
                    .await?;
                group.name
            }
        };

        // TODO: correct email of the sender
        let mut from = self.kernel_service.config().mail.newsletter_address.clone();
        from.name = name;

        let mut schedule_for = Utc::now();
        let one_sec = Duration::seconds(1);

        for (i, contact) in contacts.into_iter().filter(|c| !c.email.is_empty()).enumerate() {
            // every 12 emails we increase by one second in order to not spam the outbound email server
            if i % 12 == 0 {
                schedule_for = schedule_for + one_sec;
            }

            let to = mail::Address {
                name: contact.name,
                address: contact.email,
            };
            let subscription_id = match subscriptions.get(&contact.id) {
                Some(sub_id) => *sub_id,
                None => continue,
            };
            let job = Message::InboxSendNewsletterMessage {
                message_id,
                from: from.clone(),
                to,
                subscription_id: Some(subscription_id),
            };
            match self.queue.push(job, Some(schedule_for)).await {
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
