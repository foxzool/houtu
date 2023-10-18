use std::collections::HashMap;

use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

use crate::specification::common::definitions::NumericValue;
use crate::specification::common::RootProperty;

/// Statistics about property values.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PropertyStatistics {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The minimum property value occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the minimum of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub min: Option<NumericValue>,
    /// The maximum property value occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the maximum of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub max: Option<NumericValue>,
    /// The arithmetic mean of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the mean of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub mean: Option<NumericValue>,
    /// The median of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the median of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub median: Option<NumericValue>,
    /// The standard deviation of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the standard deviation of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: Option<NumericValue>,
    /// The variance of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the variance of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    pub variance: Option<NumericValue>,
    /// The sum of property values occurring in the tileset.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the sum of all property values,
    /// after the transforms based on the `normalized`,
    /// `offset`, and `scale` properties have been applied.
    pub sum: Option<NumericValue>,
    /// A dictionary, where each key corresponds to an enum `name` and each value is the number of occurrences of that enum. Only applicable when `type` is `ENUM`. For fixed-length arrays, this is an array of component-wise occurrences.
    pub occurrences: Option<HashMap<String, OccurrencesValue>>,
}

#[derive(Debug, PartialEq)]
pub enum OccurrencesValue {
    Integer(i64),
    Array(Vec<i64>),
}

impl<'de> Deserialize<'de> for OccurrencesValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(number) => {
                if let Some(integer) = number.as_i64() {
                    Ok(OccurrencesValue::Integer(integer))
                } else {
                    Err(serde::de::Error::custom(format!(
                        "Expected integer, got {}",
                        number
                    )))
                }
            }
            serde_json::Value::Array(array) => {
                let mut result = Vec::with_capacity(array.len());
                for value in array {
                    if let Some(integer) = value.as_i64() {
                        result.push(integer);
                    } else {
                        return Err(serde::de::Error::custom(format!(
                            "Expected integer, got {}",
                            value
                        )));
                    }
                }
                Ok(OccurrencesValue::Array(result))
            }
            _ => Err(serde::de::Error::custom(format!(
                "Expected integer or array, got {}",
                value
            ))),
        }
    }
}

impl Serialize for OccurrencesValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OccurrencesValue::Integer(integer) => serializer.serialize_i64(*integer),
            OccurrencesValue::Array(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::specification::common::definitions::NumericValue;
    use crate::specification::properties_statistics::{OccurrencesValue, PropertyStatistics};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_occurrences_value() {
        let json = json!(1);
        let occurrences_value: OccurrencesValue = serde_json::from_value(json).unwrap();
        assert_eq!(occurrences_value, OccurrencesValue::Integer(1));
        let json = json!([1, 2, 3]);
        let occurrences_value: OccurrencesValue = serde_json::from_value(json).unwrap();
        assert_eq!(occurrences_value, OccurrencesValue::Array(vec![1, 2, 3]));
    }

    #[test]
    fn test_property_statistics() {
        let json = json!(
            {
                "min": 1,
                "max": 2,
                "mean": 3,
                "median": 4,
                "standardDeviation": 5,
                "variance": 6,
                "sum": 7,
                "occurrences": {
                    "example_ENUM": 1
                }
            }
        );
        let property_statistics: PropertyStatistics = serde_json::from_value(json).unwrap();
        assert_eq!(property_statistics.min, Some(NumericValue::Numeric(1.0)));
        assert_eq!(property_statistics.max, Some(NumericValue::Numeric(2.0)));
        assert_eq!(property_statistics.mean, Some(NumericValue::Numeric(3.0)));
        assert_eq!(property_statistics.median, Some(NumericValue::Numeric(4.0)));
        assert_eq!(
            property_statistics.standard_deviation,
            Some(NumericValue::Numeric(5.0))
        );
        assert_eq!(
            property_statistics.variance,
            Some(NumericValue::Numeric(6.0))
        );
        assert_eq!(property_statistics.sum, Some(NumericValue::Numeric(7.0)));
        assert_eq!(
            property_statistics.occurrences,
            Some({
                let mut map = HashMap::new();
                map.insert("example_ENUM".to_owned(), OccurrencesValue::Integer(1));
                map
            })
        );
    }
}
