use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContact {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewsletterList {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewsletterMessage {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteContact {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNewsletterList {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNewsletterMessage {
    // TODO
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
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrash {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArchive {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpam {
    // TODO
}
