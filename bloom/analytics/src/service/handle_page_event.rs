use super::Service;
use kernel::{
    domain::{analytics::events::PageEvent, messages::Message},
    Actor,
};
use stdx::{chrono::Utc, log::error};

impl Service {
    pub async fn handle_page_event(&self, actor: Actor, mut event: PageEvent) -> Result<(), kernel::Error> {
        let anonymous_id = self.kernel_service.current_anonymous_id(actor)?;
        event.anonymous_id = anonymous_id;
        event.received_at = Utc::now();

        let job = Message::AnalyticsPageEvent(event);
        match self.queue.push(job, None).await {
            Err(err) => {
                error!("kernel.handle_page_event: queueing event: {}", &err);
                Err(err)
            }
            Ok(_) => Ok(()),
        }
    }
}
