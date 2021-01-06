use kernel::Service;
use std::sync::Arc;

pub struct Scheduler {
    kernel_service: Arc<Service>,
}

impl Scheduler {
    pub fn new(kernel_service: Arc<Service>) -> Self {
        Scheduler { kernel_service }
    }
}
