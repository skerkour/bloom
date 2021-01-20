use super::{SendNewsletterMessageJobInput, Service};

impl Service {
    pub async fn job_send_newsletter_message(&self, input: SendNewsletterMessageJobInput) -> Result<(), kernel::Error> {
        todo!()
    }
}
