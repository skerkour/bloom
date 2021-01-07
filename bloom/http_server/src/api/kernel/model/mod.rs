use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Id,
    created_at: Time,
    name: String,
}

impl From<kernel::entities::User> for User {
    fn from(_item: kernel::entities::User) -> Self {
        unimplemented!(); // TODO
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationStarted {
    pub pending_user_id: Id,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Id,
    pub created_at: Time,
    pub token: Option<String>,
}

impl From<kernel::entities::Session> for Session {
    fn from(_item: kernel::entities::Session) -> Self {
        unimplemented!(); // TODO
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedIn {
    pub session: Option<Session>,
    pub me: Option<User>,
    pub two_fa_method: Option<TwoFaMethod>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TwoFaMethod {
    Totp,
}

impl From<kernel::consts::TwoFaMethod> for TwoFaMethod {
    fn from(item: kernel::consts::TwoFaMethod) -> Self {
        match item {
            kernel::consts::TwoFaMethod::Totp => TwoFaMethod::Totp,
        }
    }
}

impl From<kernel::service::SignedIn> for SignedIn {
    fn from(item: kernel::service::SignedIn) -> Self {
        match item {
            kernel::service::SignedIn::Success { session, user } => SignedIn {
                session: Some(session.into()),
                me: Some(user.into()),
                two_fa_method: None,
            },
            kernel::service::SignedIn::TwoFa(method) => SignedIn {
                session: None,
                me: None,
                two_fa_method: Some(method.into()),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignInStarted {
    pub pending_session_id: Id,
}

impl From<kernel::entities::PendingSession> for SignInStarted {
    fn from(item: kernel::entities::PendingSession) -> Self {
        SignInStarted {
            pending_session_id: item.id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Success {
    pub success: bool,
}

impl From<bool> for Success {
    fn from(item: bool) -> Self {
        Success { success: item }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetupTwoFa {
    pub base64_qr_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    // TODO
}

impl From<kernel::entities::Group> for Group {
    fn from(_item: kernel::entities::Group) -> Self {
        unimplemented!(); // TODO
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvitation {
    // TODO
}

impl From<kernel::entities::GroupInvitation> for GroupInvitation {
    fn from(_item: kernel::entities::GroupInvitation) -> Self {
        unimplemented!(); // TODO
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignedStorageUrl {
    pub url: String,
    pub key: String,
    pub size: u64,
}

impl From<kernel::service::SignedStorageUrl> for SignedStorageUrl {
    fn from(item: kernel::service::SignedStorageUrl) -> Self {
        SignedStorageUrl {
            url: item.url,
            key: item.key,
            size: item.size,
        }
    }
}
