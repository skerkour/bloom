use std::collections::HashMap;

use sqlx::types::Json;
use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
    uuid::Uuid,
};

// TODO: improve
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Visitor {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub anonymous_id: Uuid,

    // 	ContactID *uuid.UUID `db:"contact_id"`
    pub namespace_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct TrackEvent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
    pub sent_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,

    pub event_name: String,
    pub properties: Json<HashMap<String, String>>,

    pub url: String,
    pub user_agent: String,
    pub referrer: String,
    pub device_type: DeviceType,
    pub country: String,
    pub country_code: String,
    pub os_name: String,
    pub os_version: String,
    pub browser_name: String,
    pub browser_version: String,
    pub path: String,
    pub screen_width: i64,
    pub screen_height: i64,

    pub visitor_id: Uuid,
    pub namespace_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PageEvent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
    pub sent_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,

    pub page_name: String,

    pub url: String,
    pub user_agent: String,
    pub referrer: String,
    pub device_type: DeviceType,
    pub country: String,
    pub country_code: String,
    pub os_name: String,
    pub os_version: String,
    pub browser_name: String,
    pub browser_version: String,
    pub path: String,
    pub screen_width: i64,
    pub screen_height: i64,

    pub visitor_id: Uuid,
    pub namespace_id: Uuid,
}

#[derive(sqlx::Type, Debug, Clone, Copy, Eq, PartialEq)]
#[sqlx(rename_all = "snake_case")]
pub enum DeviceType {
    Phone,
    Tablet,
    Desktop,
    Other,
}
