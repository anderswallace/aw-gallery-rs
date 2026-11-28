use worker::{D1Database, Result, wasm_bindgen::JsValue};

use crate::services::photo_service::PhotoUpload;

pub struct PhotoDb {
    db: D1Database,
}

impl PhotoDb {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    pub async fn create_pending(&self, id: &str, key: &str, photo: &PhotoUpload) -> Result<()> {
        let statement = self.db.prepare(
            "INSERT INTO photos (
                id,
                object_key,
                filename,
                content_type,
                camera,
                film,
                status
            ) VALUES (?, ?, ?, ?, ?, ?, 'pending')",
        );

        let statement = statement.bind(&[
            JsValue::from_str(id),
            JsValue::from_str(key),
            JsValue::from(&photo.content_type),
            JsValue::from_str(&photo.filename),
            JsValue::from_str(&photo.camera),
            photo
                .film
                .as_deref()
                .map(JsValue::from_str)
                .unwrap_or(JsValue::NULL),
        ])?;

        statement.run().await?;

        Ok(())
    }

    pub async fn mark_completed(&self, id: &str) -> Result<()> {
        self.db
            .prepare(
                "UPDATE photos
                SET status = 'completed'
                WHERE id = ? AND status = 'pending'",
            )
            .bind(&[JsValue::from_str(id)])?
            .run()
            .await?;

        Ok(())
    }
}
