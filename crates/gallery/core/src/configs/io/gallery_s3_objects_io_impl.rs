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
pub enum GalleryS3ObjectsIoImpl {
    #[default]
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "aws")]
    Aws,
}

impl TryFrom<String> for GalleryS3ObjectsIoImpl {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "aws" => Ok(Self::Aws),
            "null" => Ok(Self::Null),
            _ => bail!("Invalid Gallery S3Objects IO impl: '{value}'"),
        }
    }
}

impl GalleryS3ObjectsIoImpl {
    pub const ENV_VAR: &'static str = "GALLERY_S3_OBJECT_IO_IMPL";
}
