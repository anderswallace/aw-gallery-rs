use anyhow::Result;
use derive_more::Constructor;
use std::sync::Arc;

use crate::{
    data::{
        photo::image_upload::ImageUpload,
        s3::{s3_object_key::S3ObjectKey, s3_presigned_url::S3PresignedUrl},
    },
    io::s3_objects::s3_objects_io::S3ObjectsIO,
    services::s3_objects::s3_objects_service::S3ObjectsService,
};

/// Orchestrator implementation of S3ObjectsService
#[derive(Constructor)]
pub struct OrchestratorS3ObjectsService {
    s3_objects_io: Arc<dyn S3ObjectsIO>,
}

#[async_trait::async_trait]
impl S3ObjectsService for OrchestratorS3ObjectsService {
    async fn upload_file(&self, image: ImageUpload) -> Result<S3ObjectKey> {
        let object_key = self.s3_objects_io.upload_file(image).await?;
        Ok(object_key)
    }

    async fn generate_presigned_url(&self, key: S3ObjectKey) -> Result<S3PresignedUrl> {
        let presigned_url = self.s3_objects_io.generate_presigned_url(key).await?;
        Ok(presigned_url)
    }
}
