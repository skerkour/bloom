use crate::api::scalars::{Id, Time};
use kernel::consts;
use serde::{Deserialize, Serialize};

pub mod input;

pub fn convert_user(user: kernel::entities::User, private_details: bool) -> User {
    let mut ret = User {
        id: None,
        created_at: None,
        name: user.name,
        username: user.username,
        namespace_id: None,
        avatar_url: String::from(consts::DEFAULT_AVATAR), // TODO
    };

    if private_details {
        ret.id = Some(user.id);
        ret.created_at = Some(user.created_at);
        ret.namespace_id = Some(user.namespace_id);
    }

    ret
}

pub fn convert_group(group: kernel::entities::Group, private_details: bool) -> Group {
    let mut ret = Group {
        id: None,
        created_at: None,
        namespace_id: None,
        name: group.name,
        avatar_url: String::from(consts::DEFAULT_AVATAR), // TODO
    };

    if private_details {
        ret.id = Some(group.id);
        ret.created_at = Some(group.created_at);
        ret.namespace_id = Some(group.namespace_id);
    }

    ret
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Id>,
    pub created_at: Option<Time>,
    pub avatar_url: String,
    pub name: String,
    pub username: String,
    pub namespace_id: Option<Id>,
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
            user: convert_user(me.user, true),
            groups: me.groups.into_iter().map(|g| convert_group(g, true)).collect(),
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
    fn from(input: kernel::service::Registered) -> Self {
        SignedIn {
            me: Some(Me {
                user: convert_user(input.user, true),
                session: input.session.into(),
                groups: Vec::new(),
            }),
            token: Some(input.token),
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
    pub id: Option<Id>,
    pub created_at: Option<Time>,
    pub avatar_url: String,
    pub namespace_id: Option<Id>,
    pub name: String,
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
pub struct SignedUploadUrl {
    pub url: String,
    pub upload_id: Id,
}

impl From<kernel::service::SignedUploadUrl> for SignedUploadUrl {
    fn from(item: kernel::service::SignedUploadUrl) -> Self {
        SignedUploadUrl {
            url: item.url,
            upload_id: item.upload_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QrcCode {
    pub base64_jpeg_qr_code: String,
}
