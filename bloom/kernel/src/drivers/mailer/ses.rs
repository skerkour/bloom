use super::{Email, Mailer};
use crate::Error;

#[derive(Debug, Clone)]
pub struct SesMailer {}

impl SesMailer {
    pub fn new() -> SesMailer {
        SesMailer {}
    }
}

#[async_trait::async_trait]
impl Mailer for SesMailer {
    async fn send(&self, _email: Email) -> Result<(), Error> {
        unimplemented!("not implemented"); // TODO
    }
}
