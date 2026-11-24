use worker::{Bucket, D1Database};

pub struct PhotoService {
    db: D1Database,
    bucket: Bucket,
}

impl PhotoService {
    pub fn new(db: D1Database, bucket: Bucket) -> Self {
        Self { db, bucket }
    }
}
