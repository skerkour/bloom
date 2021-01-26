use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;
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

#[derive(sqlx::Type, Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    Phone,
    Tablet,
    Desktop,
    Other,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Visit {
    pub date: String,
    pub views: i64,
    pub visitors: i64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub url: String,
    pub path: String,
    pub views: i64,
    pub visitors: i64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Referrer {
    pub referrer: String,
    pub views: i64,
    pub visitors: i64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub device_type: DeviceType,
    pub views: i64,
    pub visitors: i64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub event_name: String,
    pub views: i64,
    pub visitors: i64,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Os {
//     pub os: String,
//     pub views: i64,
//     pub visitors: i64,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Browser {
//     pub browser: String,
//     pub views: i64,
//     pub visitors: i64,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Country {
//     pub country: String,
//     pub country_code: String,
//     pub views: i64,
//     pub visitors: i64,
// }
