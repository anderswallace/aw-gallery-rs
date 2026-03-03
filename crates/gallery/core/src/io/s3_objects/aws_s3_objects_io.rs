use anyhow::{Result, bail};
use async_trait::async_trait;
use aws_config::Region;
use aws_sdk_s3 as s3;

use crate::{
    configs::s3_config::S3Config,
    data::{
        photo::image::Image,
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
    async fn upload_file(&self, _image: Image) -> Result<S3ObjectKey> {
        bail!("TODO: Implement this function")
    }

    async fn generate_presigned_url(&self, _key: &S3ObjectKey) -> Result<S3PresignedUrl> {
        bail!("TODO: Implement this function")
    }
}
