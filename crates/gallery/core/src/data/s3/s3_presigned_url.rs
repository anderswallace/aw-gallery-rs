use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize)]
pub struct S3PresignedUrl {
    pub url: String,
    pub expires_at: DateTime<Utc>,
}
