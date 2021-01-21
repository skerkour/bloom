use crate::{domain::messages::Message, Error, Service};
use stdx::log::error;

impl Service {
    pub async fn dispatch_delete_old_data(&self) -> Result<(), Error> {
        let job = Message::KernelDeleteOldData;
        match self.queue.push(job, None).await {
            Err(err) => {
                error!("kernel.dispatch_delete_old_data: queueing message: {}", &err);
                Err(err)
            }
            Ok(_) => Ok(()),
        }
    }
}
