use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContact {
    pub namespace_id: Id,
    pub name: String,
    pub birthday: Option<Time>,
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
    pub plan: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewsletterList {
    pub namespace_id: Id,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewsletterMessage {
    pub list_id: Id,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub scheduled_for: Option<Time>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteContact {
    pub contact_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNewsletterList {
    pub list_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNewsletterMessage {
    pub message_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContacts {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendNewsletterMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTestNewsletterMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContact {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewsletterList {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewsletterMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContact {
    pub contact_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContacts {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNewsletterList {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNewsletterLists {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNewsletterMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNewsletterMessages {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatboxMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatboxPreferences {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatboxPreferences {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatboxMessages {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInbox {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrash {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArchive {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpam {
    pub namespace_id: Id,
}
