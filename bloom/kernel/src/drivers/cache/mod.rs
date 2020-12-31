use crate::Error;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Cache<V: Send + Sync>: Send + Sync + Debug {
    async fn set(&self, key: &str, value: V) -> Result<(), Error>;
    async fn get(&self, key: &str) -> Result<V, Error>;
    async fn delete(&self, key: &str) -> Result<(), Error>;
    async fn clear(&self) -> Result<(), Error>;
}
