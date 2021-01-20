use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct Email {
    pub from: Address,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
    pub reply_to: Vec<Address>,
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
    // pub attachments: Vec<EmailAttachment>,
    // headers
}

impl Email {
    pub fn bytes(&self) -> Vec<u8> {
        todo!();
    }
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub enum EmailAttachment {
//     Binary { body: Vec<u8>, filename: String, mime_type: String },
//     FromFile { path: String, filename: Option<String>, mime_type: String },
// }
