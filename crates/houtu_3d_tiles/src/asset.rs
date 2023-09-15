use crate::common::RootProperty;
use serde::{Deserialize, Serialize};

/// Metadata about the entire tileset.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The 3D Tiles version. The version defines the JSON schema for the tileset JSON and the base set of tile formats.
    pub version: String,
    /// Application-specific version of this tileset, e.g., for when an existing tileset is updated.
    pub tileset_version: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_asset() {
        let json = json!({
            "version": "1.0"
        });
        let asset: super::Asset = serde_json::from_value(json).unwrap();
        assert_eq!(asset.version, "1.0");
        assert_eq!(asset.tileset_version, None);
    }
}
