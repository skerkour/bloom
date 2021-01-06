use kernel::{drivers::Queue, Error, Service};
use std::sync::Arc;

mod scheduler;

pub async fn run(queue: Arc<dyn Queue>, kernel_service: Arc<Service>) -> Result<(), Error> {
    Ok(())
}
