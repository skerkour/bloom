use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompleteRegistration {
    pub pending_user_id: Id,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    pub email_or_username: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompleteSignIn {
    pub pending_session_id: Id,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct RevokeSession {
    pub session_id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyEmail {
    pub pending_email_id: Id,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteMyAccount {
    pub two_fa_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompleteTwoFaSetup {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct DisableTwoFa {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompleteTwoFaChallenge {
    pub pending_session_id: Id,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateMyProfile {
    pub username: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub path: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteGroup {
    pub group_id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateGroupProfile {
    pub group_id: Id,
    pub name: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
    // pub avatar: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct QuitGroup {
    pub group_id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct InvitePeopleInGroup {
    pub group_id: Id,
    pub usernames: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AcceptGroupInvitation {
    pub invitation_id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct DeclineGroupInvitation {
    pub invitation_id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct CancelGroupInvitation {
    pub invitation_id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveMemberFromGroup {
    pub group_id: Id,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignedUploadUrl {
    pub namespace_id: Id,
    pub filesize: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateQrCode {
    pub input: String,
}
