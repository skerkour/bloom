use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
    uuid::Uuid,
};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct TrackEvent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PageEvent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
