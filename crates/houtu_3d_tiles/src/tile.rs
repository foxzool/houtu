use serde::{Deserialize, Serialize};

use crate::bounding_volume::BoundingVolume;
use crate::content::Content;
use crate::implicit_tiling::ImplicitTiling;
use crate::metadata_entity::MetaDataEntity;

/// A tile in a 3D Tiles tileset.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    /// The bounding volume that encloses the tile.
    #[serde(rename = "boundingVolume")]
    pub bounding_volume: Option<BoundingVolume>,
    /// Optional bounding volume that defines the volume the viewer shall be inside of before the tile's content
    /// will be requested and before the tile will be refined based on geometricError.
    #[serde(rename = "viewerRequestVolume")]
    pub viewer_request_volume: Option<BoundingVolume>,
    /// The error, in meters, introduced if this tile is rendered and its children are not. At runtime,
    /// the geometric error is used to compute screen space error (SSE), i.e., the error measured in pixels.
    #[serde(rename = "geometricError")]
    pub geometric_error: Option<f64>,
    /// Specifies if additive or replacement refinement is used when traversing the tileset for rendering.
    /// This property is required for the root tile of a tileset; it is optional for all other tiles.
    /// The default is to inherit from the parent tile.
    pub refine: Option<Refine>,
    /// A floating-point 4x4 affine transformation matrix, stored in column-major order, that transforms the tile's content--i.e.,
    /// its features as well as content.boundingVolume, boundingVolume, and viewerRequestVolume--from the tile's local coordinate system to the parent tile's coordinate system, or, in the case of a root tile, from the tile's local coordinate system to the tileset's coordinate system. `transform` does not apply to any volume property when the volume is a region, defined in EPSG:4979 coordinates.
    /// `transform` scales the `geometricError` by the maximum scaling factor from the matrix.
    pub transform: Option<[f64; 16]>,
    /// Metadata about the tile's content and a link to the content. When this is omitted the tile is just used for culling. When this is defined, then `contents` shall be undefined.
    pub content: Option<Content>,
    /// An array of contents. When this is defined, then `content` shall be undefined.
    pub contents: Option<Vec<Content>>,
    /// A metadata entity that is associated with this tile.
    pub metadata: Option<MetaDataEntity>,
    /// An object that describes the implicit subdivision of this tile.
    #[serde(rename = "implicitTiling")]
    pub implicit_tiling: Option<ImplicitTiling>,
    /// An array of objects that define child tiles.
    /// Each child tile content is fully enclosed by its parent tile's bounding volume and, generally,
    /// has a geometricError less than its parent tile's geometricError.
    /// For leaf tiles, the length of this array is zero, and children may not be defined.
    pub children: Option<Vec<Tile>>,
}

#[derive(Debug)]
pub enum Refine {
    ADD,
    REPLACE,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for Refine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ADD" => Ok(Refine::ADD),
            "REPLACE" => Ok(Refine::REPLACE),
            _ => Ok(Refine::Other(value)),
        }
    }
}

impl serde::Serialize for Refine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Refine::ADD => serializer.serialize_str("ADD"),
            Refine::REPLACE => serializer.serialize_str("REPLACE"),
            Refine::Other(value) => serializer.serialize_str(value),
        }
    }
}
