use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use aws_config::Region;
use aws_sdk_s3::{self as s3, presigning::PresigningConfig};
use s3::primitives::ByteStream;

use crate::{
    configs::s3_config::S3Config,
    data::{
        photo::image_upload::ImageUpload,
        s3::{s3_object_key::S3ObjectKey, s3_presigned_url::S3PresignedUrl},
    },
    io::s3_objects::s3_objects_io::S3ObjectsIO,
};

/// AWS Implementation of S3 Objects IO
pub struct AwsS3ObjectsIO {
    client: s3::Client,
    bucket: String,
}

impl AwsS3ObjectsIO {
    pub async fn new(config: &S3Config) -> Result<Self> {
        let sdk_config = aws_config::from_env()
            .region(Region::new(config.region.clone()))
            .load()
            .await;
        let client = s3::Client::new(&sdk_config);

        Ok(Self {
            client,
            bucket: config.bucket.clone(),
        })
    }
}

#[async_trait]
impl S3ObjectsIO for AwsS3ObjectsIO {
    async fn upload_file(&self, image: ImageUpload) -> Result<S3ObjectKey> {
        let body = ByteStream::from(image.bytes);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&image.filename)
            .body(body)
            .send()
            .await?;

        Ok(S3ObjectKey::new(&image.filename)?)
    }

    async fn generate_presigned_url(&self, key: S3ObjectKey) -> Result<S3PresignedUrl> {
        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key.as_str())
            .presigned(
                PresigningConfig::builder()
                    .expires_in(Duration::from_secs(60))
                    .build()
                    .expect("valid presign config"),
            )
            .await?;

        Ok(S3PresignedUrl {
            url: presigned.uri().to_string(),
        })
    }
}
