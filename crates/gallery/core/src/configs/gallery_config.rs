use std::env::var;
use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::classes::configs::has_gallery_io_config::HasGalleryIoConfig;
use crate::classes::configs::has_gallery_services_config::HasGalleryServicesConfig;
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

impl HasGalleryServicesConfig for GalleryConfig {
    fn gallery_services_config(&self) -> &GalleryServicesConfig {
        &self.services
    }
}

impl HasGalleryIoConfig for GalleryConfig {
    fn gallery_io_config(&self) -> &GalleryIoConfig {
        &self.io
    }
}

impl GalleryConfig {
    pub fn load() -> Result<Self> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("gallery.yml");

        let contents = read_to_string(&path)
            .with_context(|| format!("Error finding Gallery yaml file at {}", path.display()))?;

        let mut config: Self =
            serde_yml::from_str(&contents).context("Error parsing Gallery yaml file")?;

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
