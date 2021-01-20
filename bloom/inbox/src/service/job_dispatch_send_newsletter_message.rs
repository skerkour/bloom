use super::Service;
use stdx::uuid::Uuid;

impl Service {
    pub async fn job_dispatch_send_newsletter_message(&self, message_id: Uuid) -> Result<(), kernel::Error> {
        todo!()
    }
}
