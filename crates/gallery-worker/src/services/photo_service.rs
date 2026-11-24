use crate::infrastructure::photo_bucket::PhotoBucket;
use uuid::Uuid;
use worker::{D1Database, Result};

pub struct PhotoUpload {
    pub filename: String,
    pub content_type: String,
    pub camera: String,
    pub film: Option<String>,
    pub bytes: Vec<u8>,
}

pub struct PhotoService {
    db: D1Database,
    bucket: PhotoBucket,
}

impl PhotoService {
    pub fn new(db: D1Database, bucket: PhotoBucket) -> Self {
        Self { db, bucket }
    }

    // upload image to R2 bucket and D1 DB
    pub async fn upload(&self, photo: PhotoUpload) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let key = format!("photos/{id}.webp");

        self.bucket.upload(key, photo).await?;

        Ok("ok".to_string())
    }
}
