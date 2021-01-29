use lettre::message::header::ContentType;
use lettre::Message;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("mail: ParseAddressError")]
    ParseAddressError,
    #[error("mail: EmailBuildError")]
    EmailBuildError,
}

impl std::convert::From<lettre::address::AddressError> for Error {
    fn from(_: lettre::address::AddressError) -> Self {
        Error::ParseAddressError
    }
}

impl std::convert::From<lettre::error::Error> for Error {
    fn from(_: lettre::error::Error) -> Self {
        Error::EmailBuildError
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub address: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self
            .name
            .chars()
            .into_iter()
            .filter(|c| c.is_alphanumeric() || *c == ' ')
            .collect::<String>()
            .trim()
            .to_string();

        let address = self.address.clone();
        write!(f, "{} <{}>", name, address)
    }
}

impl FromStr for Address {
    type Err = Error;

    /// TODO: improve, not safe to use with externally provided data
    fn from_str(input: &str) -> Result<Address, Error> {
        let mut parts: Vec<String> = input.split(' ').map(|part| part.to_string()).collect();

        if parts.len() < 2 {
            return Err(Error::ParseAddressError);
        }

        let address = parts
            .remove(parts.len() - 1)
            .trim_start_matches("<")
            .trim_end_matches(">")
            .to_string();

        let name = parts
            .join(" ")
            .chars()
            .into_iter()
            .filter(|c| c.is_alphanumeric() || *c == ' ')
            .collect::<String>()
            .trim()
            .to_string();

        Ok(Address {
            name,
            address,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    pub from: Address,
    pub to: Address,
    // pub cc: Vec<Address>,
    // pub bcc: Vec<Address>,
    pub reply_to: Option<Address>,
    pub subject: String,
    // pub text: Option<String>,
    pub html: String,
    // pub attachments: Vec<EmailAttachment>,
    // headers
}

impl Email {
    pub fn bytes(&self) -> Result<Vec<u8>, Error> {
        let mut builder = Message::builder()
            .header(ContentType::html())
            .from(self.from.to_string().parse()?)
            .to(self.to.to_string().parse()?)
            .subject(&self.subject);

        if let Some(ref reply_to) = self.reply_to {
            builder = builder.reply_to(reply_to.to_string().parse()?)
        }

        // TODO: single / multipart builder?
        let email = builder.body(self.html.clone())?;

        Ok(email.formatted())
    }
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub enum EmailAttachment {
//     Binary { body: Vec<u8>, filename: String, mime_type: String },
//     FromFile { path: String, filename: Option<String>, mime_type: String },
// }
