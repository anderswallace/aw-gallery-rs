use crate::infrastructure::photo_bucket::PhotoBucket;
use crate::infrastructure::photo_db::PhotoDb;
use uuid::Uuid;
use worker::Result;

pub struct PhotoUpload {
    pub filename: String,
    pub content_type: String,
    pub camera: String,
    pub film: Option<String>,
    pub bytes: Vec<u8>,
}

pub struct PhotoService {
    db: PhotoDb,
    bucket: PhotoBucket,
}

impl PhotoService {
    pub fn new(db: PhotoDb, bucket: PhotoBucket) -> Self {
        Self { db, bucket }
    }

    // upload image to R2 bucket and D1 DB
    pub async fn upload(&self, photo: PhotoUpload) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let key = format!("photos/{id}.webp");

        // Create pending entry in D1
        self.db.create_pending(&id, &key, &photo).await?;

        // Upload image to R2
        self.bucket.upload(key, photo).await?;

        // Update entry in D1 with completed status
        self.db.mark_completed(&id).await?;

        Ok(())
    }
}
