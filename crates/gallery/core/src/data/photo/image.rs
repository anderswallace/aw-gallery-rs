use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, JsonSchema,
)]
pub struct Image {
    id: String,
    camera: String,
    film: String,
    file: String,
}
