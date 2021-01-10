mod create_page_event;
mod create_track_event;
mod create_visitor;
mod find_devices;
mod find_events;
mod find_pages;
mod find_referrers;
mod find_visitor_by_anonymous_id;
mod find_visits;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
