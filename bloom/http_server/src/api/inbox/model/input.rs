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
pub struct DeleteContact {
    pub contact_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContacts {
    pub namespace_id: Id,
    pub list_id: Option<Id>,
    pub contacts_csv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContact {
    pub contact_id: Id,
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
pub struct GetContact {
    pub contact_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContacts {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    pub conversation_id: Id,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatboxMessage {
    pub namespace_id: Id,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatboxPreferences {
    pub namespace_id: Id,
    pub color: String,
    pub name: String,
    pub show_branding: bool,
    pub welcome_message: String,
    pub twitter: String,
    pub facebook_url: String,
    pub instagram: String,
    pub whatsapp_number: String,
    pub mastodon_url: String,
    pub website_url: String,
    pub telegram: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatboxPreferences {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatboxMessages {
    pub namespace_id: Id,
    pub after: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInbox {
    pub namespace_id: Id,
    pub after: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrash {
    pub namespace_id: Id,
    pub after: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArchive {
    pub namespace_id: Id,
    pub after: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpam {
    pub namespace_id: Id,
    pub after: Option<Id>,
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
pub struct LinkChatboxContact {
    pub namespace_id: Id,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveConversation {
    pub conversation_id: Id,
}
