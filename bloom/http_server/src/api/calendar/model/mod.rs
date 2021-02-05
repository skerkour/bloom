use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Id,
    pub created_at: Time,
    pub updated_at: Time,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_at: Time,
    pub end_at: Time,
    pub namespace_id: Id,
}

impl From<calendar::entities::Event> for Event {
    fn from(event: calendar::entities::Event) -> Self {
        Event {
            id: event.id,
            created_at: event.created_at,
            updated_at: event.updated_at,
            title: event.title,
            description: event.description,
            location: event.location,
            start_at: event.start_at,
            end_at: event.end_at,
            namespace_id: event.namespace_id,
        }
    }
}
