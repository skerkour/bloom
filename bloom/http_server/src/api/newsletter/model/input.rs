use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateList {
    pub namespace_id: Id,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessage {
    pub list_id: Id,
    pub name: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteList {
    pub list_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessage {
    pub message_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    pub message_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTestMessage {
    pub message_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateList {
    pub list_id: Id,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessage {
    pub message_id: Id,
    pub list_id: Id,
    pub name: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetList {
    pub list_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLists {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessage {
    pub message_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessages {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeToList {
    pub name: Option<String>,
    pub email: String,
    pub list_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeFromList {
    pub subscription_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveContactFromList {
    pub contact_id: Id,
    pub list_id: Id,
}
