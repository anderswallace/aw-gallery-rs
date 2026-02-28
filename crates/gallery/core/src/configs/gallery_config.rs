use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::configs::gallery_io_config::GalleryIoConfig;
use crate::configs::gallery_services_config::GalleryServicesConfig;

/// Root configuration for the Gallery core
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema,
)]
#[serde(rename = "camelCase", deny_unknown_fields)]
pub struct GalleryConfig {
    pub io: GalleryIoConfig,
    pub services: GalleryServicesConfig,
}

impl Default for GalleryConfig {
    fn default() -> Self {
        Self {
            io: GalleryIoConfig::default(),
            services: GalleryServicesConfig::default(),
        }
    }
}
