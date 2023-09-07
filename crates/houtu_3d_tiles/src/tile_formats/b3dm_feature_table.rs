use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::tile_formats::feature_table::BinaryBodyOffset;

/// A set of Batched 3D Model semantics that contain additional information about features in a tile.
#[derive(Debug, Serialize, Deserialize)]
pub struct B3dmFeatureTable {
    /// A GlobalPropertyInteger object defining an integer property for all features.
    /// Details about this property are described in the 3D Tiles specification.
    #[serde(rename = "BATCH_LENGTH")]
    pub batch_length: BatchLength,
    /// A GlobalPropertyCartesian3 object defining a 3-component numeric property for all features.
    /// Details about this property are described in the 3D Tiles specification.
    #[serde(rename = "RTC_CENTER")]
    pub rtc_center: Option<RtcCenter>,
}

impl ExtensibleObject for B3dmFeatureTable {
    const TYPE_NAME: &'static str = "B3dmFeatureTable";
}

#[derive(Debug, Serialize, PartialEq)]
pub enum BatchLength {
    BinaryBodyOffset(BinaryBodyOffset),
    Integer(i64),
}

impl<'de> Deserialize<'de> for BatchLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => Ok(BatchLength::BinaryBodyOffset(binary_body_offset)),
                    Err(_) => Err(serde::de::Error::custom("byteOffset must be defined")),
                }
            }
            serde_json::Value::Number(number) => match number.as_i64() {
                None => {
                    let number = number.as_f64().unwrap();
                    Ok(BatchLength::Integer(number as i64))
                }
                Some(integer) => Ok(BatchLength::Integer(integer)),
            },
            _ => Err(serde::de::Error::custom(
                "byteOffset or integer must be defined",
            )),
        }
    }
}

/// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
#[derive(Debug, Serialize, PartialEq)]
pub enum RtcCenter {
    BinaryBodyOffset(BinaryBodyOffset),
    Cartesian3([f64; 3]),
}

impl<'de> Deserialize<'de> for RtcCenter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => Ok(RtcCenter::BinaryBodyOffset(binary_body_offset)),
                    Err(_) => Err(serde::de::Error::custom("byteOffset must be defined")),
                }
            }
            serde_json::Value::Array(value) => {
                if value.len() == 3 {
                    let mut array = [0.0; 3];
                    for (i, v) in value.iter().enumerate() {
                        if let Some(v) = v.as_f64() {
                            array[i] = v;
                        } else {
                            return Err(serde::de::Error::custom("Invalid array"));
                        }
                    }
                    Ok(RtcCenter::Cartesian3(array))
                } else {
                    Err(serde::de::Error::custom("Invalid array"))
                }
            }
            _ => Err(serde::de::Error::custom(
                "byteOffset, cartesian3 must be defined",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn batch_length_is_integer() {
        let json = json!(1);
        let batch_length: BatchLength = serde_json::from_value(json).unwrap();
        assert_eq!(batch_length, BatchLength::Integer(1));

        let json = json!(-1);
        let batch_length: BatchLength = serde_json::from_value(json).unwrap();
        assert_eq!(batch_length, BatchLength::Integer(-1));
    }

    #[test]
    fn batch_length_is_binary_body_offset() {
        let json = json!({
            "byteOffset": 1,
        });
        let batch_length: BatchLength = serde_json::from_value(json).unwrap();
        assert_eq!(
            batch_length,
            BatchLength::BinaryBodyOffset(BinaryBodyOffset { byte_offset: 1 })
        );
    }

    #[test]
    fn b3dm_feature_table() {
        let json = json!(
            {
                "BATCH_LENGTH": 1,
                "RTC_CENTER": [1.0, 2.0 ,3.0]
            }
        );
        let b3dm_feature_table: B3dmFeatureTable = serde_json::from_value(json).unwrap();

        assert_eq!(b3dm_feature_table.batch_length, BatchLength::Integer(1));
        assert_eq!(
            b3dm_feature_table.rtc_center,
            Some(RtcCenter::Cartesian3([1.0, 2.0, 3.0]))
        );
    }

    #[test]
    fn rtc_center_can_be_none() {
        let json = json!({
            "BATCH_LENGTH": 1,
            "RTC_CENTER": null
        });
        let b3dm_feature_table: B3dmFeatureTable = serde_json::from_value(json).unwrap();

        assert_eq!(b3dm_feature_table.rtc_center, None)
    }

    #[test]
    fn rtc_center_can_be_cart3() {
        let json = json!({
            "BATCH_LENGTH": 1,
            "RTC_CENTER": [1.0, 2.0 ,3.0]
        });
        let b3dm_feature_table: B3dmFeatureTable = serde_json::from_value(json).unwrap();

        assert_eq!(
            b3dm_feature_table.rtc_center,
            Some(RtcCenter::Cartesian3([1.0, 2.0, 3.0]))
        )
    }

    #[test]
    fn rtc_center_can_be_binary_body_offset() {
        let json = json!({
            "BATCH_LENGTH": 1,
            "RTC_CENTER": {
                "byteOffset": 1,
            }
        });
        let b3dm_feature_table: B3dmFeatureTable = serde_json::from_value(json).unwrap();

        assert_eq!(
            b3dm_feature_table.rtc_center,
            Some(RtcCenter::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 1
            }))
        )
    }
}
