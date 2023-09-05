use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// Statistics about property values.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyStatistics {
    /// The minimum property value occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the minimum of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub min: Option<serde_json::Value>,
    /// The maximum property value occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the maximum of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub max: Option<serde_json::Value>,
    /// The arithmetic mean of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the mean of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub mean: Option<serde_json::Value>,
    /// The median of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the median of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub median: Option<serde_json::Value>,
    /// The standard deviation of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the standard deviation of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: Option<serde_json::Value>,
    /// The variance of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the variance of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub variance: Option<serde_json::Value>,
    /// The sum of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the sum of all property values,
    /// after the transforms based on the `normalized`,
    /// `offset`, and `scale` properties have been applied.
    pub sum: Option<serde_json::Value>,
    /// A dictionary, where each key corresponds to an enum `name` and each value is the number of occurrences of that enum. Only applicable when `type` is `ENUM`. For fixed-length arrays, this is an array of component-wise occurrences.
    pub occurrences: Option<HashMap<String, serde_json::Value>>,
}

impl ExtensibleObject for PropertyStatistics {
    const TYPE_NAME: &'static str = "PropertyStatistics";
}
