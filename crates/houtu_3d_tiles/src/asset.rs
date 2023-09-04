use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    /// The 3D Tiles version. The version defines the JSON schema for the tileset JSON and the base
    /// set of tile formats.
    pub version: String,
    /// Application-specific version of this tileset, e.g., for when an existing tileset is
    /// updated.
    #[serde(rename = "tilesetVersion")]
    pub tileset_version: Option<String>,
    /// Dictionary object with extension-specific objects.
    pub extensions: Option<Value>,
    /// Application-specific data.
    pub extras: Option<Value>,
}
