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
        todo!(); // TODO
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
}

impl From<kernel::entities::Session> for Session {
    fn from(session: kernel::entities::Session) -> Self {
        Session {
            id: session.id,
            created_at: session.created_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Registered {
    pub session: Session,
    pub me: User,
    pub token: String,
}

impl From<kernel::service::Registered> for Registered {
    fn from(item: kernel::service::Registered) -> Self {
        Registered {
            session: item.session.into(),
            me: item.user.into(),
            token: item.token,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedIn {
    pub session: Option<Session>,
    pub me: Option<User>,
    pub two_fa_method: Option<TwoFaMethod>,
    pub token: Option<String>,
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
            kernel::service::SignedIn::Success {
                session,
                user,
                token,
            } => SignedIn {
                session: Some(session.into()),
                me: Some(user.into()),
                two_fa_method: None,
                token: Some(token),
            },
            kernel::service::SignedIn::TwoFa(method) => SignedIn {
                session: None,
                me: None,
                two_fa_method: Some(method.into()),
                token: None,
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
        Success {
            success: item,
        }
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
    pub id: Id,
    pub created_at: Time,
}

impl From<kernel::entities::Group> for Group {
    fn from(group: kernel::entities::Group) -> Self {
        Group {
            id: group.id,
            created_at: group.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvitation {
    // TODO
}

impl From<kernel::entities::GroupInvitation> for GroupInvitation {
    fn from(_item: kernel::entities::GroupInvitation) -> Self {
        todo!(); // TODO
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignedStorageUrl {
    pub url: String,
    pub upload_id: Id,
}

impl From<kernel::service::SignedStorageUrl> for SignedStorageUrl {
    fn from(item: kernel::service::SignedStorageUrl) -> Self {
        SignedStorageUrl {
            url: item.url,
            upload_id: item.upload_id,
        }
    }
}
