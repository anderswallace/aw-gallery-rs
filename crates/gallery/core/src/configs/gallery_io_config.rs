use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::configs::{io::gallery_s3_objects_io_impl::GalleryS3ObjectsIoImpl, s3_config::S3Config};

/// Configuration that selects the implementation for each Gallery IO used by Gallery
#[derive(
    Debug, Clone, PartialEq, PartialOrd, Ord, Deserialize, Serialize, Eq, Hash, JsonSchema, Default,
)]
#[serde(rename = "camelCase", deny_unknown_fields)]
pub struct GalleryIoConfig {
    pub s3_objects: GalleryS3ObjectsIoImpl,
    pub s3: S3Config,
}
