use serde::{Deserialize, Serialize};
use stdx::{
    chrono::{DateTime, Utc},
    sqlx,
    uuid::Uuid,
};

#[derive(sqlx::Type, Debug, Clone, Eq, PartialEq, Copy, Deserialize, Serialize)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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
    pub trashed_at: Option<DateTime<Utc>>,
    pub last_message_at: DateTime<Utc>,
    pub is_spam: bool,
    pub name: String,
    pub description: String,
    pub anonymous_id: Option<Uuid>,

    pub namespace_id: Uuid,
    // pub contact_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub received_at: DateTime<Utc>,
    // Body     string `db:"body"`
    pub body_html: String,
    pub from_operator: bool,

    pub conversation_id: Uuid,
    pub author_id: Option<Uuid>,
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
    pub avatar_id: Option<String>,

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
    pub subject: String,
    pub body: String,
    pub body_html: String,
    pub status: NewsletterMessageStatus,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub last_sent_at: Option<DateTime<Utc>>,
    pub sent_count: i64,
    pub error_count: i64,

    pub list_id: Uuid,
    pub namespace_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ChatboxPreferences {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub color: String,
    pub name: String,
    pub avatar_id: Option<String>,
    pub show_branding: bool,
    pub welcome_message: String,
    pub twitter: String,
    pub facebook_url: String,
    pub instagram: String,
    pub whatsapp_number: String,
    pub mastodon_url: String,
    pub website_url: String,
    pub telegram: String,

    pub namespace_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ConversationContactRelation {
    pub contact_id: Uuid,
    pub conversation_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ContactAnonymousIdRelation {
    pub contact_id: Uuid,
    pub anonymous_id: Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NewsletterListSubscription {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub contact_id: Uuid,
    pub list_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportedContact {
    pub name: String,
    pub email: String,
}
