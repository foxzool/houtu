use serde::{Deserialize, Serialize};

/// A dictionary object of metadata about per-feature properties.
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    /// The maximum value of this property of all the features in the tileset.
    pub maximum: f64,
    /// The minimum value of this property of all the features in the tileset.
    pub minimum: f64,
}
