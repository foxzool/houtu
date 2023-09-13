use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::tile_formats::feature_table::{GlobalPropertyCartesian3, GlobalPropertyInteger};

/// A set of Batched 3D Model semantics that contain additional information about features in a tile.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct B3dmFeatureTable {
    /// A GlobalPropertyInteger object defining an integer property for all features.
    /// Details about this property are described in the 3D Tiles specification.
    pub batch_length: GlobalPropertyInteger,
    /// A GlobalPropertyCartesian3 object defining a 3-component numeric property for all features.
    /// Details about this property are described in the 3D Tiles specification.
    pub rtc_center: Option<GlobalPropertyCartesian3>,
}

impl ExtensibleObject for B3dmFeatureTable {
    const TYPE_NAME: &'static str = "B3dmFeatureTable";
}

#[cfg(test)]
mod tests {
    use crate::tile_formats::feature_table::BinaryBodyOffset;
    use serde_json::json;

    use super::*;

    #[test]
    fn batch_length_is_integer() {
        let json = json!(1);
        let batch_length: GlobalPropertyInteger = serde_json::from_value(json).unwrap();
        assert_eq!(batch_length, GlobalPropertyInteger::Integer(1));

        let json = json!(-1);
        let batch_length: GlobalPropertyInteger = serde_json::from_value(json).unwrap();
        assert_eq!(batch_length, GlobalPropertyInteger::Integer(-1));
    }

    #[test]
    fn batch_length_is_binary_body_offset() {
        let json = json!({
            "byteOffset": 1,
        });
        let batch_length: GlobalPropertyInteger = serde_json::from_value(json).unwrap();
        assert_eq!(
            batch_length,
            GlobalPropertyInteger::BinaryBodyOffset(BinaryBodyOffset { byte_offset: 1 })
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

        assert_eq!(
            b3dm_feature_table.batch_length,
            GlobalPropertyInteger::Integer(1)
        );
        assert_eq!(
            b3dm_feature_table.rtc_center,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
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
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
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
            Some(GlobalPropertyCartesian3::BinaryBodyOffset(
                BinaryBodyOffset { byte_offset: 1 }
            ))
        )
    }
}
