use crate::data::db::db_record::DbRecord;
use anyhow::Result;

// High level service for DB operations
#[async_trait::async_trait]
pub trait DbService: Send + Sync + 'static {
    // Placeholder DB method
    async fn db_method(&self) -> Result<DbRecord>;
}
