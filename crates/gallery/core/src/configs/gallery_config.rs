use std::env::var;
use std::fs::read_to_string;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::configs::gallery_io_config::GalleryIoConfig;
use crate::configs::gallery_services_config::GalleryServicesConfig;

/// Root configuration for the Gallery core
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GalleryConfig {
    pub io: GalleryIoConfig,
    pub services: GalleryServicesConfig,
}

impl GalleryConfig {
    pub fn load() -> Result<Self> {
        let contents = read_to_string("gallery.yml")?;
        let mut config: Self = serde_yml::from_str(&contents)?;

        if let Ok(region) = var("AWS_REGION") {
            config.io.s3.region = region;
        }

        if let Ok(bucket) = var("AWS_BUCKET") {
            config.io.s3.bucket = bucket;
        }

        config.io.s3.validate_field()?;
        Ok(config)
    }
}
