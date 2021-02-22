use inbox::entities::NewsletterMessageStatus;
use serde::{Deserialize, Serialize};

use crate::api::scalars::{Id, Time};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Id,
    pub name: String,
    pub email: String,
}

impl From<inbox::entities::Contact> for Contact {
    fn from(contact: inbox::entities::Contact) -> Self {
        Contact {
            id: contact.id,
            name: contact.name,
            email: contact.email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub id: Id,
    pub created_at: Time,
    pub name: String,
    pub description: String,
}

impl From<inbox::entities::NewsletterList> for List {
    fn from(list: inbox::entities::NewsletterList) -> Self {
        List {
            id: list.id,
            created_at: list.created_at,
            name: list.name,
            description: list.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWithDetails {
    pub list: List,
    pub contacts: Vec<Contact>,
    pub messages: Vec<Message>,
    pub acquisition: Vec<ListAcquisition>,
}

impl From<inbox::service::NewsletterListWithDetails> for ListWithDetails {
    fn from(list: inbox::service::NewsletterListWithDetails) -> Self {
        ListWithDetails {
            list: list.list.into(),
            contacts: list.contacts.into_iter().map(Into::into).collect(),
            messages: list.messages.into_iter().map(Into::into).collect(),
            acquisition: list.acquisition.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
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
}

impl From<inbox::entities::NewsletterMessage> for Message {
    fn from(message: inbox::entities::NewsletterMessage) -> Self {
        Message {
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageWithLists {
    pub message: Message,
    pub list: List,
    pub lists: Vec<List>,
}

impl From<inbox::service::NewsletterMessageWithLists> for MessageWithLists {
    fn from(message: inbox::service::NewsletterMessageWithLists) -> Self {
        MessageWithLists {
            message: message.message.into(),
            list: message.list.into(),
            lists: message.lists.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAcquisition {
    pub date: String,
    pub new_contacts: i64,
}

impl From<inbox::service::NewsletterListAcquisition> for ListAcquisition {
    fn from(input: inbox::service::NewsletterListAcquisition) -> Self {
        ListAcquisition {
            date: input.date,
            new_contacts: input.new_contacts,
        }
    }
}
