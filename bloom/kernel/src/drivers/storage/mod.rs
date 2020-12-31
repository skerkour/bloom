use std::fmt::Debug;

pub mod s3;

#[async_trait::async_trait]
pub trait Storage: Send + Sync + Debug {}
