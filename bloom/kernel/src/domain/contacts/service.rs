use crate::Error;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Service: Send + Sync + Debug {
    async fn create_contact(&self, input: CreateContactInput) -> Result<super::Contact, Error>;
}

pub struct CreateContactInput {
    pub email: Option<String>,
}
