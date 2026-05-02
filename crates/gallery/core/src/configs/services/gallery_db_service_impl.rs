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
pub enum GalleryDbServiceImpl {
    #[serde(rename = "null")]
    #[default]
    Null,
}

impl TryFrom<String> for GalleryDbServiceImpl {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "null" => Ok(Self::Null),
            _ => bail!("Invalid Gallery DB Service Impl: '{value}'"),
        }
    }
}

impl GalleryDbServiceImpl {
    pub const ENV_VAR: &'static str = "GALLERY_DB_SERVICE_IMPL";
}
