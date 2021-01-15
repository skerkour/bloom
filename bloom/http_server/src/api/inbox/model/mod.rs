use serde::{Deserialize, Serialize};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    // TODO
}

impl From<inbox::entities::Contact> for Contact {
    fn from(_item: inbox::entities::Contact) -> Self {
        todo!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    // TODO
}

impl From<inbox::entities::Message> for Message {
    fn from(_item: inbox::entities::Message) -> Self {
        todo!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterList {
    // TODO
}

impl From<inbox::entities::NewsletterList> for NewsletterList {
    fn from(_item: inbox::entities::NewsletterList) -> Self {
        todo!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterMessage {
    // TODO
}

impl From<inbox::entities::NewsletterMessage> for NewsletterMessage {
    fn from(_item: inbox::entities::NewsletterMessage) -> Self {
        todo!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatboxMessage {
    // TODO
}

impl From<inbox::entities::Message> for ChatboxMessage {
    fn from(_item: inbox::entities::Message) -> Self {
        todo!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    // TODO
}

impl From<inbox::entities::Conversation> for Conversation {
    fn from(_item: inbox::entities::Conversation) -> Self {
        todo!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatboxPreferences {
    // TODO
}

impl From<inbox::entities::ChatboxPreferences> for ChatboxPreferences {
    fn from(_item: inbox::entities::ChatboxPreferences) -> Self {
        todo!();
    }
}
