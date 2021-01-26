use crate::{config::Config, Error};
use rusoto_core::{
    credential::{AwsCredentials, DefaultCredentialsProvider, ProvideAwsCredentials},
    Region,
};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    CopyObjectRequest, DeleteObjectRequest, GetObjectRequest, HeadObjectRequest, PutObjectRequest, S3Client,
    StreamingBody, S3,
};
use std::{fmt, path::Path, time::Duration};
use stdx::tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct S3Storage {
    s3_client: S3Client,
    base_path: String,
    bucket: String,
    region: Region,
    credentials: AwsCredentials,
}

impl fmt::Debug for S3Storage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "S3Storage{{ base_path: {}, bucker: {} }}",
            &self.base_path, &self.bucket
        )
    }
}

impl S3Storage {
    pub async fn new(config: &Config) -> S3Storage {
        let s3_client = S3Client::new(config.s3.region_rusoto.clone());

        let credentials = DefaultCredentialsProvider::new().unwrap().credentials().await.unwrap();

        S3Storage {
            s3_client,
            base_path: config.storage.base_directory.clone(),
            bucket: config.s3.bucket.clone(),
            region: config.s3.region_rusoto.clone(),
            credentials,
        }
    }
}

#[async_trait::async_trait]
impl super::Storage for S3Storage {
    async fn base_path(&self) -> String {
        self.base_path.clone()
    }

    async fn copy_object(&self, from: &str, to: &str) -> Result<(), Error> {
        let from = Path::new(&self.bucket)
            .join(&self.base_path)
            .join(from)
            .to_string_lossy()
            .to_string();
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
        let key = Path::new(&self.base_path).join(key).to_string_lossy().to_string();

        let req = GetObjectRequest {
            bucket: self.bucket.clone(),
            key,
            ..Default::default()
        };
        let res = self.s3_client.get_object(req).await?;

        let mut reader = res
            .body
            .ok_or(Error::Internal(String::from("s3.get_object: body is missing")))?
            .into_async_read();
        let mut data = Vec::with_capacity(res.content_length.unwrap_or(100_000) as usize);
        reader.read_to_end(&mut data).await?;

        Ok(data)
    }

    async fn get_object_download_url(&self, key: &str, name: &str, content_type: &str) -> String {
        let key = Path::new(&self.base_path).join(key).to_string_lossy().to_string();

        // sanitize filename
        let name: String = name
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .collect();

        let options = PreSignedRequestOption {
            expires_in: Duration::from_secs(4 * 3600), // 4 hours
        };
        let req = GetObjectRequest {
            bucket: self.bucket.clone(),
            key,
            response_content_type: Some(content_type.to_string()),
            response_content_disposition: Some(format!("attachment; filename=\"{}\"", name)),
            ..Default::default()
        };

        req.get_presigned_url(&self.region, &self.credentials, &options)
    }

    async fn get_object_size(&self, key: &str) -> Result<i64, Error> {
        let key = Path::new(&self.base_path).join(key).to_string_lossy().to_string();

        let req = HeadObjectRequest {
            bucket: self.bucket.clone(),
            key,
            ..Default::default()
        };

        let res = self.s3_client.head_object(req).await?;

        Ok(res.content_length.ok_or(Error::Internal(String::from(
            "s3.get_object_size: content_length is missing",
        )))?)
    }

    async fn get_presigned_upload_url(&self, key: &str, size: u64) -> String {
        let key = Path::new(&self.base_path).join(key).to_string_lossy().to_string();

        let options = PreSignedRequestOption {
            expires_in: Duration::from_secs(4 * 3600), // 4 hours
        };
        let req = PutObjectRequest {
            bucket: self.bucket.clone(),
            key,
            content_length: Some(size as i64),
            ..Default::default()
        };

        req.get_presigned_url(&self.region, &self.credentials, &options)
    }

    async fn put_object(&self, key: &str, object: Vec<u8>, content_type: &str) -> Result<(), Error> {
        let key = Path::new(&self.base_path).join(key).to_string_lossy().to_string();

        let req = PutObjectRequest {
            bucket: self.bucket.clone(),
            key,
            content_length: Some(object.len() as i64),
            content_type: Some(content_type.to_string()),
            body: Some(StreamingBody::from(object)),
            ..Default::default()
        };

        self.s3_client.put_object(req).await?;

        Ok(())
    }
}
