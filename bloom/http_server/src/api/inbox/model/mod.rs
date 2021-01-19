use inbox::entities::NewsletterMessageStatus;
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
    pub avatar: String,
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
            avatar: String::from(consts::DEFAULT_AVATAR), // TODO
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
pub struct NewsletterList {
    pub id: Id,
    pub created_at: Time,
    pub name: String,
    pub description: String,
}

impl From<inbox::entities::NewsletterList> for NewsletterList {
    fn from(list: inbox::entities::NewsletterList) -> Self {
        NewsletterList {
            id: list.id,
            created_at: list.created_at,
            name: list.name,
            description: list.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterMessage {
    pub id: Id,
    pub created_at: Time,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub body_html: String,
    pub status: NewsletterMessageStatus,
    pub scheduled_for: Option<Time>,
    pub last_sent_at: Option<Time>,
    pub sent_count: i64,
    pub error_count: i64,
    pub list_id: Id,
}

impl From<inbox::entities::NewsletterMessage> for NewsletterMessage {
    fn from(message: inbox::entities::NewsletterMessage) -> Self {
        NewsletterMessage {
            id: message.id,
            created_at: message.created_at,
            name: message.name,
            subject: message.subject,
            body: message.body,
            body_html: message.body_html,
            status: message.status,
            scheduled_for: message.scheduled_for,
            last_sent_at: message.last_sent_at,
            sent_count: message.sent_count,
            error_count: message.error_count,
            list_id: message.list_id,
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
pub struct ChatboxPreferences {
    pub color: String,
    pub name: String,
    pub avatar: String,
    pub show_branding: bool,
    pub welcome_message: String,
}

impl From<inbox::entities::ChatboxPreferences> for ChatboxPreferences {
    fn from(preferences: inbox::entities::ChatboxPreferences) -> Self {
        ChatboxPreferences {
            color: preferences.color,
            name: preferences.name,
            avatar: String::from(consts::DEFAULT_AVATAR), // TODO
            show_branding: preferences.show_branding,
            welcome_message: preferences.welcome_message,
        }
    }
}
