use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//TODO: Set struct values to newtypes when we start implementing the IO
/// Configuration for interacting with AWS bucket
#[derive(
    Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, JsonSchema, Default,
)]
#[serde(rename = "camelCase", deny_unknown_fields)]
pub struct S3Config {
    pub access_key: String,
    pub secret_access_key: String,
    pub region: String,
    pub bucket: String,
}
