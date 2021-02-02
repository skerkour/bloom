use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use stdx::{
    chrono::{DateTime, Utc},
    uuid::Uuid,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackEvent {
    pub anonymous_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sent_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub namespace_id: Uuid,

    pub event_name: String,
    pub properties: HashMap<String, String>,

    pub name: String,
    pub url: String,
    pub referrer: String,
    pub screen_width: i64,
    pub screen_height: i64,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageEvent {
    pub anonymous_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sent_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub namespace_id: Uuid,

    pub name: String,
    pub url: String,
    pub referrer: String,
    pub screen_width: i64,
    pub screen_height: i64,
    pub user_agent: String,
}
