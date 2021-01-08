use crate::repository::Repository;
use kernel::Service as KernelService;
use kernel::{db::DB, drivers};
use std::sync::Arc;

mod handle_page_event;
mod handle_track_event;
mod process_page_event;
mod process_track_event;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    kernel_service: Arc<KernelService>,
    queue: Arc<dyn drivers::Queue>,
}

impl Service {
    pub fn new(kernel_service: Arc<KernelService>, db: DB, queue: Arc<dyn drivers::Queue>) -> Service {
        let repo = Repository::new();

        Service {
            db,
            repo,
            kernel_service,
            queue,
        }
    }
}
