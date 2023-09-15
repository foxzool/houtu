use serde::{Deserialize, Deserializer, Serialize};

use crate::common::RootProperty;
use crate::metadata_entity::MetaDataEntity;
use crate::property_table::PropertyTable;

/// An object describing the availability of tiles and content in a subtree, as well as availability of children subtrees.
/// May also store metadata for available tiles and content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtree {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// An array of buffers.
    pub buffers: Vec<Buffer>,
    /// An array of buffer views.
    pub buffer_views: Vec<BufferView>,
    /// An array of property tables.
    pub property_tables: Vec<PropertyTable>,
    /// The availability of tiles in the subtree. The availability bitstream is a 1D boolean array where tiles are ordered by their level in the subtree and Morton index within that level. A tile's availability is determined by a single bit, 1 meaning a tile exists at that spatial index, and 0 meaning it does not. The number of elements in the array is `(N^subtreeLevels - 1)/(N - 1)` where N is 4 for subdivision scheme `QUADTREE` and 8 for `OCTREE`. Availability may be stored in a buffer view or as a constant value that applies to all tiles. If a non-root tile's availability is 1 its parent tile's availability shall also be 1. `tileAvailability.constant: 0` is disallowed, as subtrees shall have at least one tile.
    pub tile_availability: Availability,
    /// An array of content availability objects. If the tile has a single content this array will have one element; if the tile has multiple contents - as supported by 3DTILES_multiple_contents and 3D Tiles 1.1 - this array will have multiple elements.
    pub content_availability: Vec<Availability>,
    /// The availability of children subtrees. The availability bitstream is a 1D boolean array where subtrees are ordered by their Morton index in the level of the tree immediately below the bottom row of the subtree. A child subtree's availability is determined by a single bit, 1 meaning a subtree exists at that spatial index, and 0 meaning it does not. The number of elements in the array is `N^subtreeLevels` where N is 4 for subdivision scheme `QUADTREE` and 8 for `OCTREE`. Availability may be stored in a buffer view or as a constant value that applies to all child subtrees. If availability is 0 for all child subtrees, then the tileset does not subdivide further.
    pub child_subtree_availability: Availability,
    /// Index of the property table containing tile metadata. Tile metadata only exists for available tiles and is tightly packed by increasing tile index. To access individual tile metadata, implementations may create a mapping from tile indices to tile metadata indices.
    pub tile_metadata: Option<u64>,
    /// An array of indexes to property tables containing content metadata. If the tile has a single content this array will have one element; if the tile has multiple contents - as supported by 3DTILES_multiple_contents and 3D Tiles 1.1 - this array will have multiple elements. Content metadata only exists for available contents and is tightly packed by increasing tile index. To access individual content metadata, implementations may create a mapping from tile indices to content metadata indices.
    pub content_metadata: Vec<u64>,
    /// Subtree metadata encoded in JSON.
    pub subtree_metadata: Option<MetaDataEntity>,
}

/// A buffer is a binary blob. It is either the binary chunk of the subtree file, or an external buffer referenced by a URI.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The URI (or IRI) of the external schema file. Relative paths are relative to the file containing the buffer JSON. uri is required when using the JSON subtree format and not required when using the binary subtree format — when omitted the buffer refers to the binary chunk of the subtree file.Data URIs are not allowed.
    pub uri: Option<String>,
    /// The length of the buffer in bytes.
    pub byte_length: i64,
    /// The name of the buffer.
    pub name: Option<String>,
}

/// A contiguous subset of a buffer
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The index of the buffer.
    pub buffer: i64,
    /// The offset into the buffer in bytes.
    pub byte_offset: i64,
    /// The total byte length of the buffer view.
    pub byte_length: i64,
    /// The name of the bufferView.
    pub name: Option<String>,
}

/// An object describing the availability of a set of elements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// Index of a buffer view that indicates whether each element is available. The bitstream conforms to the boolean array encoding described in the 3D Metadata specification. If an element is available, its bit is 1, and if it is unavailable, its bit is 0.
    pub bitstream: Option<u64>,
    /// A number indicating how many 1 bits exist in the availability bitstream.
    pub available_count: Option<u64>,
    /// Integer indicating whether all of the elements are available (1) or all are unavailable (0).
    #[serde(deserialize_with = "deserialize_option_bool_from_anything")]
    pub constant: Option<bool>,
}

pub fn deserialize_option_bool_from_anything<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AnythingOrBool {
        String(String),
        Int(i64),
        Float(f64),
        Boolean(bool),
        Null,
    }

    match AnythingOrBool::deserialize(deserializer)? {
        AnythingOrBool::Boolean(b) => Ok(Some(b)),
        AnythingOrBool::Int(i) => match i {
            1 => Ok(Some(true)),
            0 => Ok(Some(false)),
            _ => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
        },
        AnythingOrBool::Float(f) => {
            if (f - 1.0f64).abs() < f64::EPSILON {
                Ok(Some(true))
            } else if f == 0.0f64 {
                Ok(Some(false))
            } else {
                Err(serde::de::Error::custom(
                    "The number is neither 1.0 nor 0.0",
                ))
            }
        }
        AnythingOrBool::String(string) => {
            if let Ok(b) = string.parse::<bool>() {
                Ok(Some(b))
            } else if let Ok(i) = string.parse::<i64>() {
                match i {
                    1 => Ok(Some(true)),
                    0 => Ok(Some(false)),
                    _ => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
                }
            } else if let Ok(f) = string.parse::<f64>() {
                if (f - 1.0f64).abs() < f64::EPSILON {
                    Ok(Some(true))
                } else if f == 0.0f64 {
                    Ok(Some(false))
                } else {
                    Err(serde::de::Error::custom(
                        "The number is neither 1.0 nor 0.0",
                    ))
                }
            } else {
                Err(serde::de::Error::custom(format!(
                    "Could not parse boolean from a string: {}",
                    string
                )))
            }
        }
        AnythingOrBool::Null => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::definitions::AnyValue;
    use serde_json::json;

    #[test]
    fn test_buffer() {
        let json = json!({
            "uri": "uri",
            "byteLength": 1,
            "name": "name"
        });
        let buffer: Buffer = serde_json::from_value(json).unwrap();
        assert_eq!(buffer.uri, Some("uri".to_owned()));
        assert_eq!(buffer.byte_length, 1);
        assert_eq!(buffer.name, Some("name".to_owned()));

        let json = json!(
            {
                "byteLength": -1,
            }
        );

        let buffer: Buffer = serde_json::from_value(json).unwrap();
        assert_eq!(buffer.uri, None);
        assert_eq!(buffer.byte_length, -1);
        assert_eq!(buffer.name, None);
    }

    #[test]
    fn test_buffer_view() {
        let json = json!({
            "buffer": 1,
            "byteOffset": 1,
            "byteLength": 1,
            "name": "name"
        });
        let buffer_view: BufferView = serde_json::from_value(json).unwrap();
        assert_eq!(buffer_view.buffer, 1);
        assert_eq!(buffer_view.byte_offset, 1);
        assert_eq!(buffer_view.byte_length, 1);
        assert_eq!(buffer_view.name, Some("name".to_owned()));

        let json = json!(
            {
                "buffer": -1,
                "byteOffset": -1,
                "byteLength": -1,
            }
        );

        let buffer_view: BufferView = serde_json::from_value(json).unwrap();
        assert_eq!(buffer_view.buffer, -1);
        assert_eq!(buffer_view.byte_offset, -1);
        assert_eq!(buffer_view.byte_length, -1);
        assert_eq!(buffer_view.name, None);
    }

    #[test]
    fn test_availability() {
        let json = json!({
            "bitstream": 1,
            "availableCount": 1,
            "constant": 1
        });
        let availability: Availability = serde_json::from_value(json).unwrap();
        assert_eq!(availability.bitstream, Some(1));
        assert_eq!(availability.available_count, Some(1));
        assert_eq!(availability.constant, Some(true));

        let json = json!(
            {
                "bitstream": -1,
                "availableCount": -1,
                "constant": null
            }
        );

        let availability = serde_json::from_value::<Availability>(json);
        assert!(availability.is_err());

        let json = json!({
            "bitstream": 1,
            "availableCount": 1,
            "constant": "true"
        });
        let availability: Availability = serde_json::from_value(json).unwrap();
        assert_eq!(availability.constant, Some(true));

        let json = json!({
            "bitstream": 1,
            "availableCount": 1,
            "constant": 1.0
        });
        let availability: Availability = serde_json::from_value(json).unwrap();
        assert_eq!(availability.constant, Some(true));

        let json = json!({
            "bitstream": 1,
            "availableCount": 1,
            "constant": false
        });
        let availability: Availability = serde_json::from_value(json).unwrap();
        assert_eq!(availability.constant, Some(false));
    }

    #[test]
    fn test_subtree() {
        let json = json!(
            {
                "buffers": [
                    {
                        "uri": "uri",
                        "byteLength": 1,
                        "name": "name"
                    }
                ],
                "bufferViews": [
                    {
                        "buffer": 1,
                        "byteOffset": 1,
                        "byteLength": 1,
                        "name": "name"
                    }
                ],
                "propertyTables": [
                    {
                        "name": "name",
                        "class": "class",
                        "count": 1,
                        "properties": {
                            "example_STRING" : {
                                "values" : 7,
                                "stringOffsets" : 8
                            }
                        }
                    }
                ],
                "tileAvailability": {
                    "bitstream": 1,
                    "availableCount": 1,
                    "constant": 1
                },
                "contentAvailability": [
                    {
                        "bitstream": 1,
                        "availableCount": 1,
                        "constant": 1
                    }
                ],
                "childSubtreeAvailability": {
                    "bitstream": 1,
                    "availableCount": 1,
                    "constant": 1
                },
                "tileMetadata": 1,
                "contentMetadata": [
                    1
                ],
                "subtreeMetadata": {
                    "class": "building",
                    "properties": {
                        "id": "id"
                    }
                }
            }
        );
        let subtree: Subtree = serde_json::from_value(json).unwrap();
        assert_eq!(subtree.buffers.len(), 1);
        assert_eq!(subtree.buffer_views.len(), 1);
        assert_eq!(subtree.property_tables.len(), 1);
        assert_eq!(subtree.tile_availability.bitstream, Some(1));
        assert_eq!(subtree.tile_availability.available_count, Some(1));
        assert_eq!(subtree.tile_availability.constant, Some(true));
        assert_eq!(subtree.content_availability.len(), 1);
        assert_eq!(subtree.child_subtree_availability.bitstream, Some(1));
        assert_eq!(subtree.child_subtree_availability.available_count, Some(1));
        assert_eq!(subtree.child_subtree_availability.constant, Some(true));
        assert_eq!(subtree.tile_metadata, Some(1));
        assert_eq!(subtree.content_metadata.len(), 1);
        assert_eq!(
            subtree.subtree_metadata.as_ref().unwrap().class,
            "building".to_string()
        );
        assert_eq!(
            subtree.subtree_metadata.as_ref().unwrap().properties["id"],
            AnyValue::String("id".to_owned())
        );
    }
}
