use anyhow::bail;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    AsRefStr,
    EnumIter,
    Serialize,
    Deserialize,
    JsonSchema,
    Default,
)]
#[serde(rename_all = "camelCase")]
pub enum GalleryS3ObjectsServiceImpl {
    #[serde(rename = "null")]
    #[default]
    Null,
    #[serde(rename = "orchestrator")]
    Orchestrator,
}

impl TryFrom<String> for GalleryS3ObjectsServiceImpl {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "orchestrator" => Ok(Self::Orchestrator),
            "null" => Ok(Self::Null),
            _ => bail!("Invalid Gallery S3 Service Impl: '{value}'"),
        }
    }
}

impl GalleryS3ObjectsServiceImpl {
    pub const ENV_VAR: &'static str = "GALLERY_S3_OBJECTS_SERVICE_IMPL";
}
