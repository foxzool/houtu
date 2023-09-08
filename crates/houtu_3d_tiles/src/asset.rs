use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// Metadata about the entire tileset.
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    /// The 3D Tiles version. The version defines the JSON schema for the tileset JSON and the base set of tile formats.
    pub version: String,
    /// Application-specific version of this tileset, e.g., for when an existing tileset is updated.
    #[serde(rename = "tilesetVersion")]
    pub tileset_version: Option<String>,
}

impl ExtensibleObject for Asset {
    const TYPE_NAME: &'static str = "Asset";
}
