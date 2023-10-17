use crate::common::RootProperty;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FeatureTable {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// An object defining the offset into a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
    pub binary_body_offset: Option<BinaryBodyOffset>,
    /// An object defining the reference to a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
    pub binary_body_reference: Option<BinaryBodyReference>,
    /// An object defining a global boolean property value for all features.
    pub global_property_boolean: Option<GlobalPropertyBoolean>,
    /// An object defining a global integer property value for all features.
    pub global_property_integer: Option<GlobalPropertyInteger>,
    /// An object defining a global numeric property value for all features.
    pub global_property_number: Option<GlobalPropertyNumber>,
    /// An object defining a global 3-component numeric property values for all features.
    pub global_property_cartesian3: Option<GlobalPropertyCartesian3>,
    /// An object defining a global 4-component numeric property values for all features.
    pub global_property_cartesian4: Option<GlobalPropertyCartesian4>,
}

/// An object defining the offset into a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryBodyOffset {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The offset into the buffer in bytes.
    pub byte_offset: u64,
}

/// An object defining the reference to a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryBodyReference {
    #[serde(flatten)]
    pub root: RootProperty,
    /// The offset into the buffer in bytes.
    pub byte_offset: u64,
    /// The datatype of components in the property. This is defined only if the semantic allows for overriding the implicit component type. These cases are specified in each tile format.
    pub component_type: ComponentType,
}

/// The datatype of components in the property. This is defined only if the semantic allows for overriding the implicit component type. These cases are specified in each tile format.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, EnumString)]
#[allow(non_camel_case_types)]
pub enum ComponentType {
    #[default]
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    INT,
    UNSIGNED_INT,
    FLOAT,
    DOUBLE,
}

/// An object defining a global integer property value for all features.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GlobalPropertyInteger {
    BinaryBodyOffset(BinaryBodyOffset),
    Integer(u64),
}

/// An object defining a global numeric property value for all features.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GlobalPropertyNumber {
    BinaryBodyOffset(BinaryBodyOffset),
    Number(f64),
}

/// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GlobalPropertyCartesian3 {
    BinaryBodyOffset(BinaryBodyOffset),
    Cartesian3([f64; 3]),
}

/// An object defining a global 4-component numeric property values for all features.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GlobalPropertyCartesian4 {
    BinaryBodyOffset(BinaryBodyOffset),
    Cartesian4([f64; 4]),
}

pub type GlobalPropertyBoolean = bool;

/// A user-defined property which specifies application-specific metadata in a tile. Values can refer to sections in the binary body with a `BinaryBodyReference` object. Global values can be also be defined directly in the JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Property {
    /// An object defining the reference to a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
    BinaryBodyReference(BinaryBodyReference),
    /// An object defining a global boolean property value for all features.
    GlobalPropertyBoolean(GlobalPropertyBoolean),
    /// An object defining a global integer property value for all features.
    GlobalPropertyInteger(GlobalPropertyInteger),
    /// An object defining a global numeric property value for all features.
    GlobalPropertyNumber(GlobalPropertyNumber),
    /// An object defining a global 3-component numeric property values for all features.
    GlobalPropertyCartesian3(GlobalPropertyCartesian3),
    /// An object defining a global 4-component numeric property values for all features.
    GlobalPropertyCartesian4(GlobalPropertyCartesian4),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_property_binary_ref() {
        let json = json!(
        {
            "byteOffset": 0,
            "componentType": "INT"
        });
        let property: Property = serde_json::from_value(json).unwrap();
        assert_eq!(
            property,
            Property::BinaryBodyReference(BinaryBodyReference {
                component_type: ComponentType::INT,
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"
        {
            "componentType": "OTHER"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(serde_json::from_value::<Property>(json_value).is_err());
    }

    #[test]
    fn test_property() {
        let json = r#"true"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(property, Property::GlobalPropertyBoolean(true));

        let json = r#"1"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyInteger(GlobalPropertyInteger::Integer(1))
        );

        let json = r#"1.0"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyNumber(GlobalPropertyNumber::Number(1.0))
        );

        let json = r#"[1.0, 2.0, 3.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyCartesian3(GlobalPropertyCartesian3::Cartesian3([
                1.0, 2.0, 3.0
            ]))
        );

        let json = r#"[1.0, 2.0, 3.0, 4.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyCartesian4(GlobalPropertyCartesian4::Cartesian4([
                1.0, 2.0, 3.0, 4.0
            ]))
        );

        let json = r#"[1.0, 2.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(serde_json::from_value::<Property>(json_value).is_err());

        let json = r#"
        {
            "byteOffset": 1.0
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();

        assert!(serde_json::from_value::<Property>(json_value).is_err());
    }

    #[test]
    fn test_global_property_integer() {
        let json = r#"
        {
            "byteOffset": 0
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyInteger = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyInteger::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"1"#;
        let property: GlobalPropertyInteger = serde_json::from_str(json).unwrap();
        assert_eq!(property, GlobalPropertyInteger::Integer(1));
    }

    #[test]
    fn test_global_property_number() {
        let json = r#"
        {
            "byteOffset": 0
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyNumber = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyNumber::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"1.0"#;
        let property: GlobalPropertyNumber = serde_json::from_str(json).unwrap();
        assert_eq!(property, GlobalPropertyNumber::Number(1.0));
    }

    #[test]
    fn test_global_property_cartesian3() {
        let json = r#"
        {
            "byteOffset": 0
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian3 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian3::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"
        {
            "byteOffset": 0,
            "componentType": "UNSIGNED_SHORT",
            "type": "SCALAR"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian3 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian3::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"
        {
            "byteOffset": 0,
            "componentType": "UNSIGNED_SHORT",
            "type": "VEC3"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian3 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian3::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = json!([1.0, 2.0, 3.0]);
        let property: GlobalPropertyCartesian3 = serde_json::from_value(json).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian3::Cartesian3([1.0, 2.0, 3.0])
        );

        let json = json!([1.0, 2.0, 3.0, 4.0]);
        assert!(serde_json::from_value::<GlobalPropertyCartesian3>(json).is_err());

        let json = json!([1.0, 2.0]);
        assert!(serde_json::from_value::<GlobalPropertyCartesian3>(json).is_err());
    }

    #[test]
    fn test_global_property_cartesian4() {
        let json = r#"
        {
            "byteOffset": 0
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian4 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian4::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"
        {
            "byteOffset": 0,
            "componentType": "UNSIGNED_SHORT",
            "type": "SCALAR"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian4 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian4::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"
        {
            "byteOffset": 0,
            "componentType": "UNSIGNED_SHORT",
            "type": "VEC3"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian4 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian4::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = r#"
        {
            "byteOffset": 0,
            "componentType": "UNSIGNED_SHORT",
            "type": "VEC4"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: GlobalPropertyCartesian4 = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian4::BinaryBodyOffset(BinaryBodyOffset {
                byte_offset: 0,
                ..Default::default()
            })
        );

        let json = json!([1.0, 2.0, 3.0, 4.0]);
        let property: GlobalPropertyCartesian4 = serde_json::from_value(json).unwrap();
        assert_eq!(
            property,
            GlobalPropertyCartesian4::Cartesian4([1.0, 2.0, 3.0, 4.0])
        );

        let json = json!([1.0, 2.0, 3.0]);
        assert!(serde_json::from_value::<GlobalPropertyCartesian4>(json).is_err());

        let json = json!([1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(serde_json::from_value::<GlobalPropertyCartesian4>(json).is_err());
    }

    #[test]
    fn test_feature_table() {
        let json = r#"
        {
            "binaryBodyOffset": {
                "byteOffset": 0
            },
            "binaryBodyReference": {
                "byteOffset": 0,
                "componentType": "UNSIGNED_SHORT"
            },
            "globalPropertyBoolean": true,
            "globalPropertyInteger": 1,
            "globalPropertyNumber": 1.0,
            "globalPropertyCartesian3": [1.0, 2.0, 3.0],
            "globalPropertyCartesian4": [1.0, 2.0, 3.0, 4.0]
        }"#;
        let feature_table: FeatureTable = serde_json::from_str(json).unwrap();
        assert_eq!(
            feature_table,
            FeatureTable {
                binary_body_offset: Some(BinaryBodyOffset {
                    byte_offset: 0,
                    ..Default::default()
                }),
                binary_body_reference: Some(BinaryBodyReference {
                    byte_offset: 0,
                    component_type: ComponentType::UNSIGNED_SHORT,
                    ..Default::default()
                }),
                global_property_boolean: Some(true),
                global_property_integer: Some(GlobalPropertyInteger::Integer(1)),
                global_property_number: Some(GlobalPropertyNumber::Number(1.0)),
                global_property_cartesian3: Some(GlobalPropertyCartesian3::Cartesian3([
                    1.0, 2.0, 3.0
                ])),
                global_property_cartesian4: Some(GlobalPropertyCartesian4::Cartesian4([
                    1.0, 2.0, 3.0, 4.0
                ])),
                ..Default::default()
            }
        );
    }
}
