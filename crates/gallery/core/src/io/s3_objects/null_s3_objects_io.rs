use anyhow::{Result, bail};
use async_trait::async_trait;

use crate::data::photo::image_upload::ImageUpload;
use crate::data::s3::s3_object_key::S3ObjectKey;
use crate::data::s3::s3_presigned_url::S3PresignedUrl;
use crate::io::s3_objects::s3_objects_io::S3ObjectsIO;

/// Null implementation that rejects all S3 operations
pub struct NullS3ObjectsIO;

#[async_trait]
impl S3ObjectsIO for NullS3ObjectsIO {
    async fn upload_file(&self, _image: ImageUpload) -> Result<S3ObjectKey> {
        bail!("S3ObjectsIO is disabled (null implementation)")
    }

    async fn generate_presigned_url(&self, _key: S3ObjectKey) -> Result<S3PresignedUrl> {
        bail!("S3ObjectsIO is disabled (null implementation)")
    }
}
