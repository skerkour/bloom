use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
    uuid::Uuid,
};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub size: i64,
    pub r#type: String,
    pub explicitly_trashed: bool,
    pub trashed_at: Option<DateTime<Utc>>,

    pub namespace_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
}
