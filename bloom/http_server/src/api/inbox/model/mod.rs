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
