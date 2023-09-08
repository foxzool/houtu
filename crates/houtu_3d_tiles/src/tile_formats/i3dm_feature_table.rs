use serde::{Deserialize, Serialize};

use crate::tile_formats::feature_table::{
    BinaryBodyReference, GlobalPropertyBoolean, GlobalPropertyCartesian3, GlobalPropertyInteger,
};

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
    /// A `GlobalPropertyInteger` object defining an integer property for all features. Details about this property are described in the 3D Tiles specification.
    pub instance_length: GlobalPropertyInteger,
    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
    pub rtc_center: Option<GlobalPropertyCartesian3>,
    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
    pub quantized_volume_offset: Option<GlobalPropertyCartesian3>,
    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
    pub quantized_volume_scale: Option<GlobalPropertyCartesian3>,
    /// A `GlobalPropertyBoolean` object defining a boolean property for all features. Details about this property are described in the 3D Tiles specification.
    pub east_north_up: Option<GlobalPropertyBoolean>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::tile_formats::feature_table::ComponentType;

    use super::*;

    #[test]
    fn i3dm_feature_table() {
        let json = json!(
            {
                "POSITION": {
                   "componentType": "BYTE"
                },
                "POSITION_QUANTIZED": {
                   "componentType": "UNSIGNED_SHORT"
                },
                "NORMAL_UP": {
                   "componentType": "FLOAT"
                },
                "NORMAL_RIGHT": {
                   "componentType": "FLOAT"
                },
                "NORMAL_UP_OCT32P": {
                   "componentType": "UNSIGNED_BYTE"
                },
                "NORMAL_RIGHT_OCT32P": {
                   "componentType": "UNSIGNED_BYTE"
                },
                "SCALE": {
                   "componentType": "FLOAT"
                },
                "SCALE_NON_UNIFORM": {
                   "componentType": "FLOAT"
                },
                "BATCH_ID": {
                   "componentType": "UNSIGNED_SHORT"
                },
                "INSTANCE_LENGTH": 1,
                "RTC_CENTER": [1.0, 2.0 ,3.0],
                "QUANTIZED_VOLUME_OFFSET": [1.0, 2.0 ,3.0],
                "QUANTIZED_VOLUME_SCALE": [1.0, 2.0 ,3.0],
                "EAST_NORTH_UP": true
            }
        );

        let i3dm_feature_table: I3dmFeatureTable = serde_json::from_value(json).unwrap();

        assert_eq!(
            i3dm_feature_table.position,
            Some(BinaryBodyReference {
                component_type: ComponentType::BYTE
            })
        );
        assert_eq!(
            i3dm_feature_table.position_quantized,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_SHORT
            })
        );
        assert_eq!(
            i3dm_feature_table.normal_up,
            Some(BinaryBodyReference {
                component_type: ComponentType::FLOAT
            })
        );
        assert_eq!(
            i3dm_feature_table.normal_right,
            Some(BinaryBodyReference {
                component_type: ComponentType::FLOAT
            })
        );
        assert_eq!(
            i3dm_feature_table.normal_up_oct32p,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_BYTE
            })
        );
        assert_eq!(
            i3dm_feature_table.normal_right_oct32p,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_BYTE
            })
        );
        assert_eq!(
            i3dm_feature_table.scale,
            Some(BinaryBodyReference {
                component_type: ComponentType::FLOAT
            })
        );
        assert_eq!(
            i3dm_feature_table.scale_non_uniform,
            Some(BinaryBodyReference {
                component_type: ComponentType::FLOAT
            })
        );
        assert_eq!(
            i3dm_feature_table.batch_id,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_SHORT
            })
        );
        assert_eq!(
            i3dm_feature_table.instance_length,
            GlobalPropertyInteger::Integer(1)
        );
        assert_eq!(
            i3dm_feature_table.rtc_center,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
        );
        assert_eq!(
            i3dm_feature_table.quantized_volume_offset,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
        );
        assert_eq!(
            i3dm_feature_table.quantized_volume_scale,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
        );
        assert_eq!(i3dm_feature_table.east_north_up, Some(true));
    }
}
