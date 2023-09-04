use crate::common::NumericValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub classes: HashMap<String, StatisticsClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsClass {
    /// The number of entities that conform to the class.
    pub count: i64,
    pub properties: HashMap<String, StatisticsClassProperty>,
}

/// Statistics about property values.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsClassProperty {
    /// The minimum property value occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub min: Option<NumericValue>,
    /// The maximum property value occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub max: Option<NumericValue>,
    /// The arithmetic mean of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the mean of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub mean: Option<NumericValue>,
    /// The median of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the median of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub median: Option<NumericValue>,
    /// The standard deviation of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the standard deviation of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: Option<NumericValue>,
    /// The variance of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the variance of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub variance: Option<NumericValue>,
    /// The sum of property values occurring in the tileset. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the sum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub sum: Option<NumericValue>,
    /// A dictionary, where each key corresponds to an enum `name` and each value is the number of occurrences of that enum. Only applicable when `type` is `ENUM`. For fixed-length arrays, this is an array of component-wise occurrences.
    pub occurrences: Option<HashMap<String, OccurrencesItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OccurrencesItem {
    Integer(i64),
    ArrayInteger(Vec<i64>),
}
