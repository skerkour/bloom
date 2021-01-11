use crate::Error;

#[derive(Debug, Clone)]
pub struct S3Storage {}

impl S3Storage {
    pub fn new() -> S3Storage {
        S3Storage {}
    }
}

#[async_trait::async_trait]
impl super::Storage for S3Storage {
    async fn get_file_size(&self, key: &str) -> Result<i64, Error> {
        todo!();
    }

    async fn copy_object(&self, from: &str, to: &str) -> Result<(), Error> {
        todo!();
    }
}
