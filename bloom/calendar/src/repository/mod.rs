#[derive(Debug)]
pub struct Repository {}

mod create_event;
mod delete_event;
mod find_event_by_id;
mod find_events;
mod update_event;

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
