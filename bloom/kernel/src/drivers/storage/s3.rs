use crate::{config::Config, Error};
use rusoto_s3::S3Client;
use std::fmt;

#[derive(Clone)]
pub struct S3Storage {
    s3_client: S3Client,
}

impl fmt::Debug for S3Storage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S3Storage{{}}")
    }
}

impl S3Storage {
    pub fn new(config: &Config) -> S3Storage {
        let s3_client = S3Client::new(config.ses.region_rusoto.clone());

        S3Storage {
            s3_client,
        }
    }
}

#[async_trait::async_trait]
impl super::Storage for S3Storage {
    async fn get_object_size(&self, key: &str) -> Result<i64, Error> {
        todo!();
    }

    async fn copy_object(&self, from: &str, to: &str) -> Result<(), Error> {
        todo!();
    }

    async fn get_presigned_uplaod_url(&self, key: &str, size: u64) -> Result<String, Error> {
        todo!();
    }
}
