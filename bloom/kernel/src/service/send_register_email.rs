use super::SendRegisterEmailInput;
use crate::{notifications, Error, Service};
use stdx::{log::error, mail};

impl Service {
    pub async fn send_register_email(&self, input: SendRegisterEmailInput) -> Result<(), Error> {
        let to = mail::Address {
            name: input.username,
            address: input.email,
        };
        let code = self.format_code_hyphen(input.code);
        let subject = format!("Bloom code: {}", &code);

        let code = self.format_code_html(code);
        let email_data = tera::Context::from_serialize(notifications::RegistrationEmailParams {
            code,
        })
        .map_err(|err| {
            error!("kernel.send_register_email: building template context: {}", &err);
            Error::Internal(err.to_string())
        })?;

        let html = self
            .templates
            .render(notifications::REGISTRATION_EMAIL_TEMPLATE_ID, &email_data)
            .map_err(|err| {
                error!("kernel.send_register_email: rendering template: {}", &err);
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
