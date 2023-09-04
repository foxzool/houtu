use crate::asset::Asset;
use crate::schema::Schema;
use crate::statistics::Statistics;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tileset {
    /// Metadata about the entire tileset.
    pub asset: Option<Asset>,
    /// A dictionary object of metadata about per-feature properties.
    pub properties: Option<serde_json::Value>,
    /// An object defining the structure of metadata classes and enums. When this is defined, then schemaUri shall be undefined.
    pub schema: Option<Schema>,
    /// The URI (or IRI) of the external schema file. When this is defined, then schema shall be undefined.
    pub schema_uri: Option<String>,
    /// An object containing statistics about metadata entities.
    pub statistics: Option<Statistics>,
    // pub groups: Option<Vec<Group>>,
}
