use super::Mailer;
use crate::{config::Config, Error};
use rusoto_ses::{RawMessage, SendRawEmailRequest, Ses, SesClient};
use std::{fmt, time::Duration};
use stdx::{mail::Email, retry};

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
    async fn send(&self, email: Email) -> Result<(), Error> {
        let raw_email = email.bytes().map_err(|_| Error::Internal)?;

        let ses_request = SendRawEmailRequest {
            raw_message: RawMessage {
                data: raw_email.into(),
            },
            ..Default::default()
        };

        retry::retry_fn(|| self.ses_client.send_raw_email(ses_request.clone()))
            .retries(10)
            .exponential_backoff(Duration::from_secs(1))
            .max_delay(Duration::from_secs(10))
            .await?;
        Ok(())
    }
}
