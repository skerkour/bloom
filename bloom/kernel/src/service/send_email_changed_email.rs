use super::SendEmailChangedEmailInput;
use crate::{notifications, Error, Service};
use stdx::{log::error, mail};

impl Service {
    pub async fn send_email_changed_email(&self, input: SendEmailChangedEmailInput) -> Result<(), Error> {
        let contact_url = self.contact_url();
        let data = tera::Context::from_serialize(notifications::EmailChangedEmailData {
            new_email: input.new_email,
            contact_url,
        })
        .map_err(|err| {
            error!("kernel.send_email_changed_email: building template context: {}", &err);
            Error::Internal(err.to_string())
        })?;
        let subject = String::from("Bloom - Your email address was updated");
        let to = mail::Address {
            name: input.name,
            address: input.email,
        };

        let html = self
            .templates
            .render(notifications::EMAIL_CHANGED_EMAIL_TEMPLATE_ID, &data)
            .map_err(|err| {
                error!("kernel.send_email_changed_email: rendering tempplate: {}", &err);
                Error::Internal(err.to_string())
            })?;

        let email = mail::Email {
            from: self.config.mail.notify_address.clone(),
            to,
            reply_to: None,
            subject,
            html,
        };
        self.mailer.send(email).await?;

        Ok(())
    }
}
