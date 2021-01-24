use crate::Error;
use std::fmt::Debug;
use stdx::mail::Email;

pub mod ses;

#[async_trait::async_trait]
pub trait Mailer: Send + Sync + Debug {
    async fn send(&self, email: Email) -> Result<(), Error>;
}

#[cfg(test)]
pub mod test {
    use super::{Email, Error, Mailer};

    #[derive(Clone, Debug)]
    pub struct MailerMock {}

    impl MailerMock {
        pub fn new() -> Self {
            MailerMock {}
        }
    }

    #[async_trait::async_trait]
    impl Mailer for MailerMock {
        async fn send(&self, _: Email) -> Result<(), Error> {
            Ok(())
        }
    }
}
