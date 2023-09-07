use serde::{Deserialize, Serialize};

use crate::tile_formats::feature_table::BinaryBodyReference;

/// A set of Instanced 3D Model semantics that contains values defining the position and appearance properties for instanced models in a tile.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct I3dmFeatureTable {
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub position: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub position_quantized: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub normal_up: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub normal_right: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub normal_up_oct32p: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub normal_right_oct32p: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub scale: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub scale_non_uniform: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub batch_id: Option<BinaryBodyReference>,
}
