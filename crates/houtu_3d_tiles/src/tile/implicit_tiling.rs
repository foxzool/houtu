use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, EnumString)]
pub enum SubdivisionScheme {
    #[strum(ascii_case_insensitive)]
    Quadtree,
    #[strum(ascii_case_insensitive)]
    Octree,
}
