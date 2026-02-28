use crate::data::photo::image::Image;
use crate::data::s3::s3_object_key::S3ObjectKey;
use crate::data::s3::s3_presigned_url::S3PresignedUrl;
use anyhow::Result;

// High level service for S3 object operations
#[async_trait::async_trait]
pub trait S3ObjectsService: Send + Sync + 'static {
    // Uploads an image to S3 and returns its S3 Object Key
    async fn upload_file(&self, image: Image) -> Result<S3ObjectKey>;

    // Generates a Pre-Signed URL using an S3 Object Key
    async fn generate_presigned_url(&self, key: S3ObjectKey) -> Result<S3PresignedUrl>;
}
