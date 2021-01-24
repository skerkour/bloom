use super::{SendNewsletterMessageJobInput, Service};
use crate::notifications;
use crate::Error;
use stdx::{chrono::Utc, log::error, mail};

impl Service {
    pub async fn job_send_newsletter_message(&self, input: SendNewsletterMessageJobInput) -> Result<(), kernel::Error> {
        let unsubscribe_link = if let Some(subscription_id) = input.subscription_id {
            Some(self.unsubscribe_link(subscription_id))
        } else {
            None
        };

        let mut message = self
            .repo
            .find_newsletter_message_by_id(&self.db, input.message_id)
            .await?;

        let data = tera::Context::from_serialize(notifications::NewsletterEmailParams {
            content: message.body_html.clone(),
            unsubscribe_link,
        })
        .map_err(|err| {
            error!("inbox.job_send_newsletter_message: building template context: {}", err);
            Error::Internal
        })?;

        let html = self
            .templates
            .render(notifications::NEWSLETTER_EMAIL_TEMPLATE_ID, &data)
            .map_err(|err| {
                error!("inbox.job_send_newsletter_message: rendering tempplate: {}", err);
                Error::Internal
            })?;

        // from := message.From
        // fromDomainParts := strings.Split(service.config.Emails.OutboundAddress.Address, "@")
        // if len(fromDomainParts) != 2 {
        //     err = errors.Internal(errMessage, err)
        //     return
        // }
        // from.Address = outboundMessage.ProjectID.String() + "@" + fromDomainParts[1]

        // headers := textproto.MIMEHeader{}

        // if message.From.Address != "" {
        //     headers["Reply-to"] = []string{message.From.Address}
        // }

        let email = mail::Email {
            from: input.from,
            to: input.to,
            reply_to: None,
            subject: message.subject.clone(),
            html,
        };
        match self.mailer.send(email).await {
            Ok(_) => {
                message.sent_count += 1;
            }
            Err(err) => {
                error!("inbox.job_send_newsletter_message: sending message: {}", err);
                message.error_count += 1;
            }
        }

        if input.subscription_id.is_some() {
            let now = Utc::now();
            message.updated_at = now;
            match self.repo.update_newsletter_message(&self.db, &message).await {
                Ok(_) => {}
                Err(err) => {
                    error!("inbox.job_send_newsletter_message: updating message: {:?}", err);
                }
            }
        }

        Ok(())
    }
}
