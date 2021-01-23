use crate::{config::Config, Error};
use rusoto_s3::{CopyObjectRequest, DeleteObjectRequest, S3Client, S3};
use std::{fmt, path::Path};

#[derive(Clone)]
pub struct S3Storage {
    s3_client: S3Client,
    base_path: String,
    bucket: String,
}

impl fmt::Debug for S3Storage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S3Storage{{ base_path: {} }}", &self.base_path)
    }
}

impl S3Storage {
    pub fn new(config: &Config) -> S3Storage {
        let s3_client = S3Client::new(config.ses.region_rusoto.clone());

        S3Storage {
            s3_client,
            base_path: config.storage.base_directory.clone(),
            bucket: config.s3.bucket.clone(),
        }
    }
}

#[async_trait::async_trait]
impl super::Storage for S3Storage {
    async fn base_path(&self) -> String {
        self.base_path.clone()
    }

    async fn copy_object(&self, from: &str, to: &str) -> Result<(), Error> {
        let from = Path::new(&self.base_path).join(from).to_string_lossy().to_string();
        let to = Path::new(&self.base_path).join(to).to_string_lossy().to_string();

        let req = CopyObjectRequest {
            bucket: self.bucket.clone(),
            key: to,
            copy_source: from,
            // content_type: Some(uploaded_file.type_),
            ..Default::default()
        };

        self.s3_client.copy_object(req).await?;

        Ok(())
    }

    async fn delete_object(&self, key: &str) -> Result<(), Error> {
        let key = Path::new(&self.base_path).join(key).to_string_lossy().to_string();

        let req = DeleteObjectRequest {
            bucket: self.bucket.clone(),
            key,
            ..Default::default()
        };

        self.s3_client.delete_object(req).await?;

        Ok(())
    }

    async fn get_object(&self, key: &str) -> Result<Vec<u8>, Error> {
        todo!();
    }

    async fn get_object_download_url(&self, key: &str, name: &str, content_type: &str) -> Result<String, Error> {
        todo!();
    }

    async fn get_object_size(&self, key: &str) -> Result<i64, Error> {
        todo!();
    }

    async fn get_presigned_upload_url(&self, key: &str, size: u64) -> Result<String, Error> {
        todo!();
    }

    async fn put_object(&self, key: &str, object: &[u8]) -> Result<(), Error> {
        todo!();
    }
}
