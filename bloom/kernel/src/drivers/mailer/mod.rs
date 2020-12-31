use crate::Error;
use std::fmt::Debug;

pub mod ses;

#[async_trait::async_trait]
pub trait Mailer: Send + Sync + Debug {
    async fn send(&self, email: Email) -> Result<(), Error>;
}

#[derive(Debug, Clone, Default)]
pub struct Email {
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub reply_to: Vec<String>,
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
    // pub attachments: Vec<EmailAttachment>,
    // headers
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub enum EmailAttachment {
//     Binary { body: Vec<u8>, filename: String, mime_type: String },
//     FromFile { path: String, filename: Option<String>, mime_type: String },
// }
