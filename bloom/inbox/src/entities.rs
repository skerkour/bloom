use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
    uuid::Uuid,
};

#[derive(sqlx::Type, Debug, Clone, Eq, PartialEq, Copy)]
#[sqlx(rename_all = "snake_case")]
pub enum NewsletterMessageStatus {
    Saved,
    Scheduled,
    Sending,
    Sent,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub archived_at: Option<DateTime<Utc>>,
    pub last_message_received_at: DateTime<Utc>,

    pub namespace_id: Uuid,
    // 	ContactID uuid.UUID `db:"contact_id"`
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,

    // Body     string `db:"body"`
    pub body_html: String,

    pub conversation_id: Uuid,
    // AuthorID       *uuid.UUID `db:"author_id"`
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

    pub name: String,
    pub from_address: String,
    pub from_name: String,
    pub subject: String,
    pub body: String,
    pub body_html: String,
    pub status: NewsletterMessageStatus,
    pub send_at: Option<DateTime<Utc>>,
    pub last_sent_at: Option<DateTime<Utc>>,
    pub sent_count: i64,
    pub error_count: i64,

    pub namespace_id: Uuid,
}
