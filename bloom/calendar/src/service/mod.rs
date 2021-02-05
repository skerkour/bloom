use crate::repository::Repository;
use kernel::db::DB;
use std::sync::Arc;
use stdx::{
    chrono::{DateTime, Utc},
    uuid::Uuid,
};

mod create_event;
mod delete_event;
mod find_events;
mod update_event;
mod validators;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    kernel_service: Arc<kernel::Service>,
}

impl Service {
    pub fn new(kernel_service: Arc<kernel::Service>, db: DB) -> Service {
        let repo = Repository::new();

        Service {
            db,
            repo,
            kernel_service,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateEventInput {
    pub namespace_id: Uuid,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpdateEventInput {
    pub event_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct DeleteEventInput {
    pub event_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindEventsInput {
    pub namespace_id: Uuid,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
}
