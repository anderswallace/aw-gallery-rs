use anyhow::{Result, bail};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: Set struct values to newtypes when we start implementing the IO
/// Configuration for interacting with AWS bucket
#[derive(
    Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct S3Config {
    pub region: String,
    pub bucket: String,
}

impl S3Config {
    fn validate_field(&self) -> Result<()> {
        if self.region.is_empty() {
            bail!("Region is not set for S3Config")
        }

        if self.bucket.is_empty() {
            bail!("Bucket is not set for S3Config")
        }

        Ok(())
    }
}
