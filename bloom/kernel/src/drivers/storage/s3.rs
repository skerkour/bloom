#[derive(Debug, Clone)]
pub struct S3Storage {}

impl S3Storage {
    pub fn new() -> S3Storage {
        S3Storage {}
    }
}

#[async_trait::async_trait]
impl super::Storage for S3Storage {}
