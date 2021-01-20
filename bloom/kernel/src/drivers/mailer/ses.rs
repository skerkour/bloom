use super::{Email, Mailer};
use crate::{config::Config, Error};
use rusoto_ses::SesClient;
use std::fmt;

#[derive(Clone)]
pub struct SesMailer {
    ses_client: SesClient,
}

impl fmt::Debug for SesMailer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SesMailer{{}}")
    }
}

impl SesMailer {
    pub fn new(config: &Config) -> SesMailer {
        let ses_client = SesClient::new(config.ses.region_rusoto.clone());

        SesMailer {
            ses_client,
        }
    }
}

#[async_trait::async_trait]
impl Mailer for SesMailer {
    async fn send(&self, _email: Email) -> Result<(), Error> {
        todo!("not implemented"); // TODO
    }
}
