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
