use serde::{Deserialize, Serialize};

use crate::specification::tile_formats::feature_table::{
    BinaryBodyReference, FeatureTable, GlobalPropertyCartesian3, GlobalPropertyCartesian4,
    GlobalPropertyInteger,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct PntsFeatureTable {
    #[serde(flatten)]
    pub feature_table: FeatureTable,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub position: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub position_quantized: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub rgba: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub rgb: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub rgb565: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub normal: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub normal_oct16p: Option<BinaryBodyReference>,
    /// A `BinaryBodyReference` object defining the reference to a section of the binary body where the property values are stored. Details about this property are described in the 3D Tiles specification.
    pub batch_id: Option<BinaryBodyReference>,
    /// A `GlobalPropertyInteger` object defining an integer property for all points. Details about this property are described in the 3D Tiles specification.
    pub points_length: GlobalPropertyInteger,
    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points. Details about this property are described in the 3D Tiles specification.
    pub rtc_center: Option<GlobalPropertyCartesian3>,
    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points. Details about this property are described in the 3D Tiles specification.
    pub quantized_volume_offset: Option<GlobalPropertyCartesian3>,
    /// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all points. Details about this property are described in the 3D Tiles specification.
    pub quantized_volume_scale: Option<GlobalPropertyCartesian3>,
    /// A `GlobalPropertyCartesian4` object defining a 4-component numeric property for all points. Details about this property are described in the 3D Tiles specification.
    pub constant_rgba: Option<GlobalPropertyCartesian4>,
    /// A `GlobalPropertyInteger` object defining an integer property for all points. Details about this property are described in the 3D Tiles specification.
    pub batch_length: Option<GlobalPropertyInteger>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::specification::tile_formats::feature_table::ComponentType;

    use super::*;

    #[test]
    fn pnts_feature_table() {
        let json = json!(
            {
                "POSITION": {
                    "byteOffset": 0,
                    "componentType": "BYTE"
                },
                "POSITION_QUANTIZED": {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_SHORT"
                },
                "RGBA": {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_BYTE"
                },
                "RGB": {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_BYTE"
                },
                "RGB565": {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_BYTE"
                },
                "NORMAL": {
                    "byteOffset": 0,
                    "componentType": "FLOAT"
                },
                "NORMAL_OCT16P": {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_BYTE"
                },
                "BATCH_ID": {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_SHORT"
                },
                "POINTS_LENGTH": 1,
                "RTC_CENTER": [1.0, 2.0 ,3.0],
                "QUANTIZED_VOLUME_OFFSET": [1.0, 2.0 ,3.0],
                "QUANTIZED_VOLUME_SCALE": [1.0, 2.0 ,3.0],
                "CONSTANT_RGBA": [1.0, 2.0 ,3.0, 4.0],
                "BATCH_LENGTH": 1
            }
        );

        let pnts_feature_table: PntsFeatureTable = serde_json::from_value(json).unwrap();

        assert_eq!(
            pnts_feature_table.position,
            Some(BinaryBodyReference {
                component_type: ComponentType::BYTE,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.position_quantized,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_SHORT,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.rgba,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_BYTE,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.rgb,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_BYTE,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.rgb565,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_BYTE,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.normal,
            Some(BinaryBodyReference {
                component_type: ComponentType::FLOAT,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.normal_oct16p,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_BYTE,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.batch_id,
            Some(BinaryBodyReference {
                component_type: ComponentType::UNSIGNED_SHORT,
                ..Default::default()
            })
        );
        assert_eq!(
            pnts_feature_table.points_length,
            GlobalPropertyInteger::Integer(1)
        );
        assert_eq!(
            pnts_feature_table.rtc_center,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
        );
        assert_eq!(
            pnts_feature_table.quantized_volume_offset,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
        );
        assert_eq!(
            pnts_feature_table.quantized_volume_scale,
            Some(GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0]))
        );
        assert_eq!(
            pnts_feature_table.constant_rgba,
            Some(GlobalPropertyCartesian4::Cartesian4([1.0, 2.0, 3.0, 4.0]))
        );
        assert_eq!(
            pnts_feature_table.batch_length,
            Some(GlobalPropertyInteger::Integer(1))
        );
    }
}
