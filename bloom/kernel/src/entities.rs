use crate::consts::{self, BillingPlan, GroupRole, NamespaceType, TaxIdType, TwoFaMethod};
use stdx::sqlx;
use stdx::{chrono, uuid};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Namespace {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub path: String,
    pub r#type: NamespaceType,
    pub used_storage: i64,
    pub plan: BillingPlan,
    pub parent_id: Option<uuid::Uuid>,
}

/// User represents a Bloom user
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub blocked_at: Option<chrono::DateTime<chrono::Utc>>,

    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub two_fa_enabled: bool,
    pub two_fa_method: Option<TwoFaMethod>,
    pub encrypted_totp_secret: Option<Vec<u8>>,
    pub totp_secret_nonce: Option<Vec<u8>>,

    pub name: String,
    pub description: String,
    pub avatar_id: Option<String>,

    pub namespace_id: uuid::Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Group {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub description: String,
    pub avatar_id: Option<String>,
    pub path: String,

    pub namespace_id: uuid::Uuid,
}

/// Session entity is a session of a bloom user. It may not necessarily map 1:1 with users' devices
/// id user has mulitple browsers / apps per device
/// a Session secret_hash is as follows: blake2b(secret=session.secret, data=session.id.bytes(), size=512 bits)
/// a Session secret (not saved in DB, only sent to client) is as follows: cryptoSecureRand(size=512 bits)
/// a Session token (not saved in DB, only sent to client) is as follows: base64(session.id.bytes() +session.secret)
/// where "+" means concatenated
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Session {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub secret_hash: Vec<u8>,

    pub user_id: uuid::Uuid,
    // 	Token *string
}

/// PendingUser entity is used to as a temporary database entry to not fill the users table with junk
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PendingUser {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub username: String,
    pub email: String,
    pub failed_attempts: i64,
    pub code_hash: String,
}

/// PendingSession entity is used to as a temporary database entry to not fill the sessions table with junk
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PendingSession {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub code_hash: String,
    pub two_fa_verified: bool,
    pub failed_attempts: i64,

    pub user_id: uuid::Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PendingEmail {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub email: String,
    pub code_hash: String,
    pub failed_attempts: i64,

    pub user_id: uuid::Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct GroupMembership {
    pub joined_at: chrono::DateTime<chrono::Utc>,

    pub role: GroupRole,

    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Customer {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub subscription_updated_at: chrono::DateTime<chrono::Utc>,
    pub plan: BillingPlan,
    pub name: String,
    pub email: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub postal_code: String, // Zip or postal code
    pub address_line1: String,
    pub address_line2: String,
    pub state: String, // State, county, province, or region.
    pub tax_id_type: Option<TaxIdType>,
    pub tax_id: Option<String>,

    pub stripe_customer_id: String,
    pub stripe_subscription_id: Option<String>,
    pub stripe_product_id: Option<String>,
    pub stripe_price_id: Option<String>,
    pub stripe_tax_id: Option<String>,
    pub stripe_default_payment_method_id: Option<String>,

    pub namespace_id: Option<uuid::Uuid>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Invoice {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub amount: i64,
    pub stripe_id: String,
    pub stripe_hosted_url: String,
    pub stripe_pdf_url: String,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,

    pub customer_id: uuid::Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct GroupInvitation {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub group_id: uuid::Uuid,
    pub inviter_id: uuid::Uuid,
    pub invitee_id: uuid::Uuid,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Upload {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub size: i64,
    pub completed: bool,

    pub namespace_id: uuid::Uuid,
}

impl Upload {
    pub fn tmp_storage_key(&self) -> String {
        format!(
            "{}/{}",
            consts::UPLOAD_TMP_STORAGE_KEY_FODLER,
            self.id.to_hyphenated().to_string()
        )
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct GroupMember {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub name: String,
    pub avatar_id: Option<String>,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub role: GroupRole,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct BillingInformation {
    pub namespace: Namespace,
    pub customer: Option<Customer>,
    pub total_storage: i64,
}
