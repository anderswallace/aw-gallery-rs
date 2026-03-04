// Image payload to upload to S3
pub struct ImageUpload {
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}
