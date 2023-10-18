use serde::{Deserialize, Serialize};

use crate::specification::bounding_volume::BoundingVolume;
use crate::specification::common::RootProperty;
use crate::specification::metadata_entity::MetaDataEntity;

/// Metadata about the tile's content and a link to the content.
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// An optional bounding volume that tightly encloses tile content.
    /// tile.boundingVolume provides spatial coherence and tile.content.boundingVolume enables tight view frustum culling.
    /// When this is omitted, tile.boundingVolume is used.
    #[serde(rename = "boundingVolume")]
    pub bounding_volume: Option<BoundingVolume>,
    /// A uri that points to tile content. When the uri is relative, it is relative to the referring tileset JSON file.
    pub uri: String,
    /// Metadata that is associated with this content.
    pub metadata: Option<MetaDataEntity>,
    /// The group this content belongs to.
    /// The value is an index into the array of `groups` that is defined for the containing tileset.
    pub group: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_content() {
        let json = json!({
            "boundingVolume": {
                "box": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]
            },
            "uri": "http://localhost:8080/tileset.json",
            "metadata": {
                "class": "class",
                "properties": {
                    "example_STRING": "string",
                    "example_BOOLEAN": true,
                    "example_ENUM": "enum",
                    "example_ARRAY": [1, 2, 3],
                    "example_VEC3": [1, 2, 3],
                    "example_MAT4": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,11, 12, 13, 14, 15, 16]
                }
            },
            "group": 1
        });
        let content: Content = serde_json::from_value(json).unwrap();
        assert_eq!(
            content.bounding_volume,
            Some(BoundingVolume {
                r#box: Some([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]),
                region: None,
                sphere: None,
                root: Default::default(),
            })
        );
        assert_eq!(content.uri, "http://localhost:8080/tileset.json");
        assert_eq!(content.metadata.unwrap().class, "class");
        assert_eq!(content.group, Some(1));
    }
}
