use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::configs::services::gallery_s3_objects_service_impl::GalleryS3ObjectsServiceImpl;

/// Configuration that selects the implementation for each Gallery service used by Gallery
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Eq, JsonSchema, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GalleryServicesConfig {
    pub s3_objects: GalleryS3ObjectsServiceImpl,
}
