use anyhow::Result;
use async_trait::async_trait;

use crate::data::photo::image::Image;
use crate::data::s3::s3_object_key::S3ObjectKey;
use crate::data::s3::s3_presigned_url::S3PresignedUrl;

/// IO interface for interacting with AWS S3
#[async_trait]
pub trait S3ObjectsIO: Send + Sync + 'static {
    /// Uploads an image to the designated S3 bucket and returns an object key
    async fn upload_file(&self, image: Image) -> Result<S3ObjectKey>;

    /// Generates a pre-signed URL using the S3 object key of an image
    async fn generate_presigned_url(&self, key: S3ObjectKey) -> Result<S3PresignedUrl>;
}
