use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::subtrees::Subtrees;

/// This object allows a tile to be implicitly subdivided.
/// Tile and content availability and metadata is stored in subtrees which are referenced externally.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImplicitTiling {
    /// A string describing the subdivision scheme used within the tileset.
    #[serde(rename = "subdivisionScheme")]
    pub subdivision_scheme: SubdivisionScheme,
    /// The number of distinct levels in each subtree.
    /// For example, a quadtree with subtreeLevels = 2 will have subtrees with 5 nodes (one root and 4 children).
    #[serde(rename = "subtreeLevels")]
    pub subtree_levels: i64,
    /// The numbers of the levels in the tree with available tiles.
    #[serde(rename = "availableLevels")]
    pub available_levels: i64,
    /// An object describing the location of subtree files.
    pub subtrees: Subtrees,
}

impl ExtensibleObject for ImplicitTiling {
    const TYPE_NAME: &'static str = "ImplicitTiling";
}

#[derive(Debug)]
pub enum SubdivisionScheme {
    Quadtree,
    Octree,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for SubdivisionScheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "QUADTREE" => Ok(SubdivisionScheme::Quadtree),
            "OCTREE" => Ok(SubdivisionScheme::Octree),
            _ => Ok(SubdivisionScheme::Other(value)),
        }
    }
}

impl serde::Serialize for SubdivisionScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SubdivisionScheme::Quadtree => serializer.serialize_str("QUADTREE"),
            SubdivisionScheme::Octree => serializer.serialize_str("OCTREE"),
            SubdivisionScheme::Other(value) => serializer.serialize_str(value),
        }
    }
}
