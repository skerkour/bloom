use crate::repository::Repository;
use kernel::{db::DB, drivers};
use std::sync::Arc;
use stdx::uuid::Uuid;

mod find_analytics;
mod find_or_create_visitor;
mod handle_page_event;
mod handle_track_event;
mod process_page_event;
mod process_track_event;
mod utils;
mod validators;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    kernel_service: Arc<kernel::Service>,
    queue: Arc<dyn drivers::Queue>,
}

impl Service {
    pub fn new(kernel_service: Arc<kernel::Service>, db: DB, queue: Arc<dyn drivers::Queue>) -> Service {
        let repo = Repository::new();

        Service {
            db,
            repo,
            kernel_service,
            queue,
        }
    }
}

#[derive(Debug)]
pub struct FindOrCreateVisitorInput {
    pub anonymous_id: Uuid,
    pub namespace_id: Uuid,
}
