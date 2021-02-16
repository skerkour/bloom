use serde::{Deserialize, Serialize};
use std::time::Duration;
use stdx::sqlx;

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TwoFaMethod {
    Totp,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Copy)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GroupRole {
    Administrator,
    Member,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Copy)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NamespaceType {
    User,
    Group,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Copy)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BillingPlan {
    Free,
    Starter,
    Pro,
    // Ultra,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Copy)]
#[sqlx(rename = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    EuVat,
}

// User
pub const TOTP_ISSUER: &str = "Bloom";
pub const TOTP_QR_CODE_SIZE: u32 = 300;
pub const TOTP_QR_JPEG_QUALITY: u8 = 85;
pub const CODE_ALPHABET: &str = "abcdefghijkmnpqrstuvwxyz234567890";
/// The length in characters of the verification code sent for registration
pub const REGISTER_CODE_LENGTH: usize = 12;
/// The length in characters of the verification code sent for signing in
pub const SIGN_IN_CODE_LENGTH: usize = 12;
// The max number of invalid code an user can try to validate its account with
pub const REGISTRATION_MAX_FAILED_ATTEMPTS: i64 = 5;
// The max number of invalid codes an user can try to validate its account with
pub const SIGN_IN_MAX_FAILED_ATTEMPTS: i64 = 5;
// The max number of invalid codes an user can try to validate its email with
pub const VERIFY_EMAIL_MAX_FAILED_ATTEMPTS: i64 = 5;
pub const USER_DESCRIPTION_MAX_LENGTH: usize = 420;
pub const USER_NAME_MIN_LENGTH: usize = 3;
pub const USER_NAME_MAX_LENGTH: usize = 42;

// Group
pub const MAX_MEMBERS_PLAN_FREE: usize = 2;
pub const MAX_MEMBERS_PLAN_STARTER: usize = 5;
pub const MAX_MEMBERS_SOFT_LIMIT: usize = 250;
pub const GROUP_DESCRIPTION_MAX_LENGTH: usize = 420;
pub const GROUP_NAME_MIN_LENGTH: usize = 3;
pub const GROUP_NAME_MAX_LENGTH: usize = 42;

// Billing
pub const STORAGE_FREE: i64 = 100_000_000; // 100MB
pub const STORAGE_STARTER: i64 = 100_000_000_000; // 50GB
pub const STORAGE_PRO: i64 = 1_000_000_000_000; // 500GB
                                                // pub const STORAGE_ULTRA: i64 = 5_000_000_000_000; // 5TB
pub const CUSTOMER_PROPERTY_MAX_LENGTH: usize = 200;

// Other
pub const SLEEP_MIN: Duration = Duration::from_millis(200);
pub const SLEEP_MAX: Duration = Duration::from_millis(500);
pub const DEFAULT_AVATAR: &str = "/assets/imgs/profile.jpg";
pub const UPLOAD_TMP_STORAGE_KEY_FODLER: &str = "uploads/tmp";
pub const UPLOAD_MAX_SIZE: u64 = 8_000_000_000; // 8GB
pub const QR_CODE_INPUT_MAX_LENGTH: usize = 1024;
pub const MARKDOWN_MAX_SIZE: usize = 42_000; // 42kb

// Namespace
pub const NAMESPACE_MIN_LENGTH: usize = 4;
pub const NAMESPACE_MAX_LENGTH: usize = 20;
pub const NAMESPACE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

pub const INVALID_NAMESPACES: &[&str] = &[
    "about",
    "accounts",
    "accounts",
    "admin",
    "administrator",
    "api",
    "apps",
    "arcade",
    "archive",
    "assets",
    "auth",
    "avatar",
    "avatars",
    "blog",
    "blogs",
    "boss",
    "bot",
    "bots",
    "bitflow",
    "calendar",
    "calendars",
    "calculator",
    "career",
    "careers",
    "code",
    "codes",
    "community",
    "communities",
    "confirm",
    "confirm-email",
    "confirm-emails",
    "confirm-list-subscription",
    "confirm-list-subscriptions",
    "confirm-subscription",
    "confirm-subscriptions",
    "contact",
    "contacts",
    "contribute",
    "contributing",
    "course",
    "courses",
    "css",
    "dashboard",
    "dashboards",
    "doc",
    "docs",
    "download",
    "downloads",
    "drive",
    "email",
    "emails",
    "explore",
    "faq",
    "feature",
    "features",
    "files",
    "font",
    "fonts",
    "form",
    "forms",
    "forum",
    "forums",
    "group",
    "groups",
    "group-invitation",
    "group-invitations",
    "hello",
    "help",
    "img",
    "imgs",
    "inbox",
    "inboxes",
    "invitation",
    "invitations",
    "invite",
    "job",
    "jobs",
    "js",
    "knowledge",
    "lib",
    "libs",
    "license",
    "licensing",
    "list",
    "lists",
    "login",
    "mail",
    "mails",
    "manage",
    "manage-preferences",
    "manage-subscription",
    "manage-subscriptions",
    "me",
    "music",
    "musics",
    "namespace",
    "namespaces",
    "news",
    "newsletter",
    "no-reply",
    "noreply",
    "notes",
    "notification",
    "notifications",
    "notify",
    "one",
    "post",
    "posts",
    "preferences",
    "pricing",
    "privacy",
    "pure",
    "qrcode",
    "qrcodes",
    "register",
    "registration",
    "robot",
    "robots",
    "root",
    "search",
    "security",
    "securityteam",
    "security-team",
    "settings",
    "shop",
    "shops",
    "sign-in",
    "signin",
    "static",
    "status",
    "store",
    "subscribe",
    "subscription",
    "subscriptions",
    "support",
    "tags",
    "task",
    "tasks",
    "terms",
    "todo",
    "todos",
    "tool",
    "tools",
    "topics",
    "trending",
    "tv",
    "unsubscribe",
    "upload",
    "uploads",
    "update",
    "upgrade",
    "user",
    "users",
    "verify-email",
    "verify-emails",
    "workflow",
    "workflows",
    "wiki",
    "wikis",
];
