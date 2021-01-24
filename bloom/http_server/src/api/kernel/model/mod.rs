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
    fn from(user: kernel::entities::User) -> Self {
        User {
            id: user.id,
            created_at: user.created_at,
            name: user.name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationStarted {
    pub pending_user_id: Id,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Me {
    pub session: Session,
    pub user: User,
    pub groups: Vec<Group>,
}

impl From<kernel::service::Me> for Me {
    fn from(me: kernel::service::Me) -> Self {
        Me {
            session: me.session.into(),
            user: me.user.into(),
            groups: me.groups.into_iter().map(Into::into).collect(),
        }
    }
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
pub struct SignedIn {
    pub me: Option<Me>,
    pub two_fa_method: Option<TwoFaMethod>,
    pub token: Option<String>,
}

impl From<kernel::service::Registered> for SignedIn {
    fn from(item: kernel::service::Registered) -> Self {
        SignedIn {
            me: Some(Me {
                user: item.user.into(),
                session: item.session.into(),
                groups: Vec::new(),
            }),
            token: Some(item.token),
            two_fa_method: None,
        }
    }
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
                me,
                token,
            } => SignedIn {
                me: Some(me.into()),
                two_fa_method: None,
                token: Some(token),
            },
            kernel::service::SignedIn::TwoFa(method) => SignedIn {
                me: None,
                token: None,
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
    id: Id,
}

impl From<kernel::entities::GroupInvitation> for GroupInvitation {
    fn from(invitation: kernel::entities::GroupInvitation) -> Self {
        GroupInvitation {
            id: invitation.id,
        }
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
