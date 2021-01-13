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

    pub name: String,
    pub birthday: Option<DateTime<Utc>>,
    pub email: String,
    pub pgp_key: String,
    pub phone: String,
    pub address: String,
    pub website: String,
    pub twitter: String,
    pub instagram: String,
    pub facebook: String,
    pub linkedin: String,
    pub skype: String,
    pub telegram: String,
    pub bloom: String,
    pub notes: String,
    pub country: String,
    pub country_code: String,
    pub plan: String,
    pub user_id: String,

    pub namespace_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NewsletterList {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub description: String,

    pub namespace_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NewsletterMessage {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
