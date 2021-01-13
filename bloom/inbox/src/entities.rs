use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
    uuid::Uuid,
};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Contact {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NewsletterList {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NewsletterMessage {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
