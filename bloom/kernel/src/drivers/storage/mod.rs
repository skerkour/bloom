use crate::Error;
use std::fmt::Debug;

pub mod s3;

#[async_trait::async_trait]
pub trait Storage: Send + Sync + Debug {
    async fn get_object_size(&self, key: &str) -> Result<i64, Error>;
    async fn copy_object(&self, from: &str, to: &str) -> Result<(), Error>;
}
