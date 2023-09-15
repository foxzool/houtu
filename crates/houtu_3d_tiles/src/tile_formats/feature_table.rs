use crate::common::RootProperty;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeatureTable {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    pub binary_body_offset: Option<BinaryBodyOffset>,
    pub binary_body_reference: Option<BinaryBodyReference>,
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
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Serialize, PartialEq)]
pub enum GlobalPropertyInteger {
    BinaryBodyOffset(BinaryBodyOffset),
    Integer(i64),
}

impl<'de> Deserialize<'de> for GlobalPropertyInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => {
                        Ok(GlobalPropertyInteger::BinaryBodyOffset(binary_body_offset))
                    }
                    Err(_) => Err(serde::de::Error::custom("byteOffset must be defined")),
                }
            }
            serde_json::Value::Number(number) => match number.as_i64() {
                None => {
                    let number = number.as_f64().unwrap();
                    Ok(GlobalPropertyInteger::Integer(number as i64))
                }
                Some(integer) => Ok(GlobalPropertyInteger::Integer(integer)),
            },
            _ => Err(serde::de::Error::custom(
                "byteOffset or integer must be defined",
            )),
        }
    }
}

/// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
#[derive(Debug, Serialize, PartialEq)]
pub enum GlobalPropertyCartesian3 {
    BinaryBodyOffset(BinaryBodyOffset),
    Cartesian3([f64; 3]),
}

impl<'de> Deserialize<'de> for GlobalPropertyCartesian3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => Ok(GlobalPropertyCartesian3::BinaryBodyOffset(
                        binary_body_offset,
                    )),
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
                    Ok(GlobalPropertyCartesian3::Cartesian3(array))
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

/// An object defining a global 4-component numeric property values for all features.
#[derive(Debug, Serialize, PartialEq)]
pub enum GlobalPropertyCartesian4 {
    BinaryBodyOffset(BinaryBodyOffset),
    Cartesian4([f64; 4]),
}

impl<'de> Deserialize<'de> for GlobalPropertyCartesian4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => Ok(GlobalPropertyCartesian4::BinaryBodyOffset(
                        binary_body_offset,
                    )),
                    Err(_) => Err(serde::de::Error::custom("byteOffset must be defined")),
                }
            }
            serde_json::Value::Array(value) => {
                if value.len() == 4 {
                    let mut array = [0.0; 4];
                    for (i, v) in value.iter().enumerate() {
                        if let Some(v) = v.as_f64() {
                            array[i] = v;
                        } else {
                            return Err(serde::de::Error::custom("Invalid array"));
                        }
                    }
                    Ok(GlobalPropertyCartesian4::Cartesian4(array))
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

pub type GlobalPropertyBoolean = bool;

/// A user-defined property which specifies application-specific metadata in a tile. Values can refer to sections in the binary body with a `BinaryBodyReference` object. Global values can be also be defined directly in the JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Property {
    /// An object defining the offset into a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.    
    Offset(BinaryBodyOffset),
    /// An object defining the reference to a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
    Reference(BinaryBodyReference),
    /// An object defining a global boolean property value for all features.
    Boolean(bool),
    /// An object defining a global integer property value for all features.
    Integer(i64),
    /// An object defining a global numeric property value for all features.
    GlobalPropertyNumber(f64),
    /// An object defining a global 3-component numeric property values for all features.
    GlobalPropertyCartesian3([f64; 3]),
    /// An object defining a global 4-component numeric property values for all features.
    GlobalPropertyCartesian4([f64; 4]),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_property_byte_offset() {
        let json = r#"
        {
            "byteOffset": 10
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::Offset(BinaryBodyOffset {
                byte_offset: 10,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_property_binary_ref() {
        let json = json!(
        {
            "componentType": "INT"
        });
        let property: Property = serde_json::from_value(json).unwrap();
        assert_eq!(
            property,
            Property::Reference(BinaryBodyReference {
                component_type: ComponentType::INT,
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
        assert_eq!(property, Property::Boolean(true));

        let json = r#"1"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(property, Property::Integer(1));

        let json = r#"1.0"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(property, Property::GlobalPropertyNumber(1.0));

        let json = r#"[1.0, 2.0, 3.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyCartesian3([1.0, 2.0, 3.0])
        );

        let json = r#"[1.0, 2.0, 3.0, 4.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyCartesian4([1.0, 2.0, 3.0, 4.0])
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
}
