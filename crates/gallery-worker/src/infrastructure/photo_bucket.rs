use worker::{Bucket, HttpMetadata, Result};

use crate::services::photo_service::PhotoUpload;

pub struct PhotoBucket {
    bucket: Bucket,
}

impl PhotoBucket {
    pub fn new(bucket: Bucket) -> Self {
        Self { bucket }
    }

    pub async fn upload(&self, key: String, photo: PhotoUpload) -> Result<()> {
        let metadata = HttpMetadata {
            content_type: Some(photo.content_type),
            ..Default::default()
        };

        self.bucket
            .put(key, photo.bytes)
            .http_metadata(metadata)
            .execute()
            .await?;

        Ok(())
    }
}
