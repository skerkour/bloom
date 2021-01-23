use crate::Error;
use std::fmt::Debug;

pub mod s3;

#[async_trait::async_trait]
pub trait Storage: Send + Sync + Debug {
    async fn base_path(&self) -> String;
    async fn copy_object(&self, from: &str, to: &str) -> Result<(), Error>;
    async fn delete_object(&self, key: &str) -> Result<(), Error>;
    async fn get_object(&self, key: &str) -> Result<Vec<u8>, Error>; // (object io.ReadCloser, err error)
    async fn get_object_download_url(&self, key: &str, name: &str, content_type: &str) -> String;
    async fn get_object_size(&self, key: &str) -> Result<i64, Error>;
    async fn get_presigned_upload_url(&self, key: &str, size: u64) -> String;
    async fn put_object(&self, key: &str, object: Vec<u8>, content_type: &str) -> Result<(), Error>; // ctx context.Context, key string, object io.Reader)
}
