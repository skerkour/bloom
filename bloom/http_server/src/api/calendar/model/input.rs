use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvent {
    pub namespace_id: Id,
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_at: Time,
    pub end_at: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub event_id: Id,
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_at: Option<Time>,
    pub end_at: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvent {
    pub event_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
    pub namespace_id: Id,
    pub start_at: Option<Time>,
    pub end_at: Option<Time>,
}
