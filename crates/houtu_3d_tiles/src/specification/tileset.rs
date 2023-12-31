use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::specification::asset::Asset;
use crate::specification::common::RootProperty;
use crate::specification::group::Group;
use crate::specification::metadata_entity::MetaDataEntity;
use crate::specification::properties::Properties;
use crate::specification::schema::Schema;
use crate::specification::statistics::Statistics;

/// A 3D Tiles tileset.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tileset {
    #[serde(flatten)]
    pub root_property: RootProperty,
    /// Metadata about the entire tileset.
    pub asset: Asset,
    /// A dictionary object of metadata about per-feature properties.
    pub properties: Option<HashMap<String, Properties>>,
    /// An object defining the structure of metadata classes and enums. When this is defined, then schemaUri shall be undefined.
    pub schema: Option<Schema>,
    /// The URI (or IRI) of the external schema file. When this is defined, then schema shall be undefined.
    pub schema_uri: Option<String>,
    /// An object containing statistics about metadata entities.
    pub statistics: Option<Statistics>,
    /// An array of groups that tile content may belong to. Each element of this array is a metadata entity that describes the group. The tile content `group` property is an index into this array.
    pub groups: Option<Vec<Group>>,
    /// A metadata entity that is associated with this tileset.
    pub metadata: Option<MetaDataEntity>,
    /// The error, in meters, introduced if this tileset is not rendered. At runtime, the geometric error is used to compute screen space error (SSE), i.e., the error measured in pixels.
    #[serde(rename = "geometricError")]
    pub geometric_error: f64,
    /// The root tile.
    pub root: crate::specification::tile::Tile,
    /// Names of 3D Tiles extensions used somewhere in this tileset.
    #[serde(rename = "extensionsUsed")]
    pub extensions_used: Option<HashSet<String>>,
    /// Names of 3D Tiles extensions required to properly load this tileset. Each element of this array shall also be contained in `extensionsUsed`.
    #[serde(rename = "extensionsRequired")]
    pub extensions_required: Option<HashSet<String>>,
}
