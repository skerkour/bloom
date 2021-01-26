use kernel::consts;
use serde::{Deserialize, Serialize};

use crate::api::scalars::{Id, Time};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Id,
    pub created_at: Time,
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
    pub country: String,
    pub country_code: String,
    pub plan: String,
    pub user_id: String,
    pub avatar_url: String,
}

impl From<inbox::entities::Contact> for Contact {
    fn from(contact: inbox::entities::Contact) -> Self {
        Contact {
            id: contact.id,
            created_at: contact.created_at,
            name: contact.name,
            birthday: contact.birthday,
            email: contact.email,
            pgp_key: contact.pgp_key,
            phone: contact.phone,
            address: contact.address,
            website: contact.website,
            twitter: contact.twitter,
            instagram: contact.instagram,
            facebook: contact.facebook,
            linkedin: contact.linkedin,
            skype: contact.skype,
            telegram: contact.telegram,
            bloom: contact.bloom,
            notes: contact.notes,
            country: contact.country,
            country_code: contact.country_code,
            plan: contact.plan,
            user_id: contact.user_id,
            avatar_url: String::from(consts::DEFAULT_AVATAR), // TODO
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Id,
    pub received_at: Time,
    pub body_html: String,
}

impl From<inbox::entities::Message> for Message {
    fn from(message: inbox::entities::Message) -> Self {
        Message {
            id: message.id,
            received_at: message.received_at,
            body_html: message.body_html,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatboxMessage {
    pub id: Id,
    pub received_at: Time,
    pub body_html: String,
}

impl From<inbox::entities::Message> for ChatboxMessage {
    fn from(message: inbox::entities::Message) -> Self {
        ChatboxMessage {
            id: message.id,
            received_at: message.received_at,
            body_html: message.body_html,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Id,
    pub created_at: Time,
    pub archived_at: Option<Time>,
    pub trashed_at: Option<Time>,
    pub last_message_at: Time,
    pub is_spam: bool,
    pub name: String,
    pub description: String,
}

impl From<inbox::entities::Conversation> for Conversation {
    fn from(conversation: inbox::entities::Conversation) -> Self {
        Conversation {
            id: conversation.id,
            created_at: conversation.created_at,
            archived_at: conversation.archived_at,
            trashed_at: conversation.trashed_at,
            last_message_at: conversation.last_message_at,
            is_spam: conversation.is_spam,
            name: conversation.name,
            description: conversation.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationWithContactsAndMessages {
    pub conversation: Conversation,
    pub contacts: Vec<Contact>,
    pub messages: Vec<Message>,
}

impl From<inbox::service::ConversationWithMessageAndContacts> for ConversationWithContactsAndMessages {
    fn from(input: inbox::service::ConversationWithMessageAndContacts) -> Self {
        ConversationWithContactsAndMessages {
            conversation: input.conversation.into(),
            contacts: input.contacts.into_iter().map(Into::into).collect(),
            messages: input.messages.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatboxPreferences {
    pub color: String,
    pub name: String,
    pub avatar: String,
    pub show_branding: bool,
    pub welcome_message: String,
    pub base_url: String,
}

impl From<inbox::service::DetailedChatboxPreferences> for ChatboxPreferences {
    fn from(input: inbox::service::DetailedChatboxPreferences) -> Self {
        ChatboxPreferences {
            color: input.preferences.color,
            name: input.preferences.name,
            avatar: String::from(consts::DEFAULT_AVATAR), // TODO
            show_branding: input.preferences.show_branding,
            welcome_message: input.preferences.welcome_message,
            base_url: input.base_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub conversations: Vec<ConversationWithContactsAndMessages>,
}
