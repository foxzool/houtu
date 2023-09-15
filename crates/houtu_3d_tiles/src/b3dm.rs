use serde::{Deserialize, Serialize};

/// A set of Batched 3D Model semantics that contain additional information about features in a tile.
#[derive(Debug, Serialize, Deserialize)]
pub struct B3dmFeatureTable {
    /// A GlobalPropertyInteger object defining an integer property for all features.
    /// Details about this property are described in the 3D Tiles specification.
    #[serde(rename = "BATCH_LENGTH")]
    pub batch_length: i64,
    /// A GlobalPropertyCartesian3 object defining a 3-component numeric property for all features.
    /// Details about this property are described in the 3D Tiles specification.
    #[serde(rename = "RTC_CENTER")]
    pub rtc_center: Option<[f64; 3]>,
}
