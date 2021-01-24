use super::SendSignInEmailInput;
use crate::{notifications, Error, Service};
use stdx::{log::error, mail};

impl Service {
    pub async fn send_sign_in_email(&self, input: SendSignInEmailInput) -> Result<(), Error> {
        let to = mail::Address {
            name: input.name,
            address: input.email,
        };
        let code = self.format_code_hyphen(input.code);
        let subject = format!("Bloom code: {}", &code);

        let code = self.format_code_html(code);
        let email_data = tera::Context::from_serialize(notifications::SingInEmailParams {
            code,
        })
        .map_err(|err| {
            error!("kernel.send_sign_in_email: building template context: {}", &err);
            Error::Internal(err.to_string())
        })?;

        let html = self
            .templates
            .render(notifications::SIGN_IN_EMAIL_TEMPLATE_ID, &email_data)
            .map_err(|err| {
                error!("kernel.send_sign_in_email: rendering tempplate: {}", &err);
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

        // err = service.signInEmailTemplate.Execute(&content, params)
        // if err != nil {
        //     logger.Error("kernel.SendSignInEmail: Executing email template", log.Err("err", err))
        //     return
        // }

        // emailToSend := email.Email{
        //     From:    service.config.Emails.NotifyAddress,
        //     To:      []mail.Address{to},
        //     Subject: subject,
        //     HTML:    content.Bytes(),
        //     Text:    content.Bytes(),
        // }
        // err = service.mailer.Send(ctx, emailToSend)
        // if err != nil {
        //     logger.Error("kernel.SendSignInEmail: Sending email", log.Err("err", err))
        // }
        // return
    }
}
