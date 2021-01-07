use stdx::{
    chrono::{DateTime, Utc},
    uuid::Uuid,
};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
