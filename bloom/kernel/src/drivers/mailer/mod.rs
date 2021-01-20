use crate::Error;
use std::fmt::Debug;
use stdx::mail::Email;

pub mod ses;

#[async_trait::async_trait]
pub trait Mailer: Send + Sync + Debug {
    async fn send(&self, email: Email) -> Result<(), Error>;
}
