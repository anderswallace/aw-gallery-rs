use crate::{
    data::{
        photo::image_upload::ImageUpload,
        s3::{s3_object_key::S3ObjectKey, s3_presigned_url::S3PresignedUrl},
    },
    services::s3_objects::s3_objects_service::S3ObjectsService,
};
use anyhow::{Result, bail};

/// Null implementation of GalleryS3ObjectsService used by the null preset
#[derive(Debug, Default)]
pub struct NullS3ObjectsService;

const ERR: &str = "Gallery S3 Objects service is unavailable in the null preset";

#[async_trait::async_trait]
impl S3ObjectsService for NullS3ObjectsService {
    async fn upload_file(&self, _image: ImageUpload) -> Result<S3ObjectKey> {
        bail!(ERR)
    }

    async fn generate_presigned_url(&self, _key: S3ObjectKey) -> Result<S3PresignedUrl> {
        bail!(ERR)
    }
}
