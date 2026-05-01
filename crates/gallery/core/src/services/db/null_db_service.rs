use crate::data::db::db_record::DbRecord;
use crate::services::db::db_service::DbService;
use anyhow::{Result, bail};

/// Null implementation of DB service used by the null preset
#[derive(Default, Debug)]
pub struct NullDbService;

const ERR: &str = "DB Service is unavailable in the Null preset";

#[async_trait::async_trait]
impl DbService for NullDbService {
    async fn db_method(&self) -> Result<DbRecord> {
        bail!(ERR);
    }
}
