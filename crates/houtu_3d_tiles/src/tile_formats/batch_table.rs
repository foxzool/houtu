use serde::{Deserialize, Serialize};

use crate::tile_formats::feature_table::ComponentType;

/// An object defining the reference to a section of the binary body of the batch table where the property values are stored if not defined directly in the JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryBodyReference {
    pub byte_offset: u64,
    /// The datatype of components in the property.
    pub component_type: ComponentType,
    #[serde(rename = "type")]
    pub type_: ScaleOrVectorType,
}

/// A set of properties defining application-specific metadata for features in a tile.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTable {
    /// An object defining the reference to a section of the binary body of the batch table where the property values are stored if not defined directly in the JSON.
    pub binary_body_reference: Option<BinaryBodyReference>,
    /// A user-defined property which specifies per-feature application-specific metadata in a tile. Values either can be defined directly in the JSON as an array, or can refer to sections in the binary body with a `BinaryBodyReference` object.
    pub property: Option<PropertySchema>,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum PropertySchema {
    BinaryBodyReference(BinaryBodyReference),
    Array(Vec<serde_json::Value>),
}

impl<'de> serde::Deserialize<'de> for PropertySchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Array(array) => Ok(PropertySchema::Array(array)),
            serde_json::Value::Object(object) => {
                let byte_offset = object.get("byteOffset");
                let component_type = object.get("componentType");
                let type_ = object.get("type");

                if let (Some(byte_offset), Some(component_type), Some(type_)) =
                    (byte_offset, component_type, type_)
                {
                    let byte_offset = byte_offset.as_u64().unwrap();
                    let component_type = serde_json::from_value(component_type.clone());
                    let type_ = serde_json::from_value(type_.clone());
                    if component_type.is_err() || type_.is_err() {
                        return Err(serde::de::Error::custom(
                            "byteOffset, componentType, type must be defined",
                        ));
                    }
                    let component_type = component_type.unwrap();
                    let type_ = type_.unwrap();

                    Ok(PropertySchema::BinaryBodyReference(BinaryBodyReference {
                        byte_offset,
                        component_type,
                        type_,
                    }))
                } else {
                    Err(serde::de::Error::custom(
                        "byteOffset, componentType, type must be defined",
                    ))
                }
            }
            _ => Err(serde::de::Error::custom(
                "byteOffset, componentType, type must be defined",
            )),
        }
    }
}

/// Specifies if the property is a scalar or vector.
#[derive(Debug, PartialEq)]
pub enum ScaleOrVectorType {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for ScaleOrVectorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SCALAR" => Ok(ScaleOrVectorType::SCALAR),
            "VEC2" => Ok(ScaleOrVectorType::VEC2),
            "VEC3" => Ok(ScaleOrVectorType::VEC3),
            "VEC4" => Ok(ScaleOrVectorType::VEC4),
            _ => Ok(ScaleOrVectorType::Other(value)),
        }
    }
}

impl serde::Serialize for ScaleOrVectorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ScaleOrVectorType::SCALAR => serializer.serialize_str("SCALAR"),
            ScaleOrVectorType::VEC2 => serializer.serialize_str("VEC2"),
            ScaleOrVectorType::VEC3 => serializer.serialize_str("VEC3"),
            ScaleOrVectorType::VEC4 => serializer.serialize_str("VEC4"),
            ScaleOrVectorType::Other(value) => serializer.serialize_str(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_batch_table() {
        let json = r#"
        {
            "binaryBodyReference": {
                "byteOffset": 0,
                "componentType": "UNSIGNED_SHORT",
                "type": "SCALAR"
            },
            "property": [
                {
                    "byteOffset": 0,
                    "componentType": "UNSIGNED_SHORT",
                    "type": "SCALAR"
                },
                    {
                    "byteOffset": 1,
                    "componentType": "OTHER",
                    "type": "OTHER"
                },
                {
                    "byteOffset": 2,
                    "componentType": "UNSIGNED_SHORT",
                    "type": "VEC3"
                },
                {
                    "byteOffset": 3,
                    "componentType": "UNSIGNED_SHORT",
                    "type": "OTHER"
                }
            ]
        }
        "#;

        let batch_table: BatchTable = serde_json::from_str(json).unwrap();
        assert!(batch_table.binary_body_reference.is_some());
        let binary_body_reference: BinaryBodyReference = batch_table.binary_body_reference.unwrap();
        assert!(batch_table.property.is_some());

        let property = if let PropertySchema::Array(property) = batch_table.property.unwrap() {
            property
        } else {
            panic!("property must be array")
        };
        assert_eq!(
            binary_body_reference.component_type,
            ComponentType::UNSIGNED_SHORT
        );
        assert_eq!(binary_body_reference.byte_offset, 0);
        assert_eq!(binary_body_reference.type_, ScaleOrVectorType::SCALAR);
        let p0: BinaryBodyReference = serde_json::from_value(property[0].clone()).unwrap();
        let p1: BinaryBodyReference = serde_json::from_value(property[1].clone()).unwrap();
        let p2: BinaryBodyReference = serde_json::from_value(property[2].clone()).unwrap();
        let p3: BinaryBodyReference = serde_json::from_value(property[3].clone()).unwrap();
        assert_eq!(p0.component_type, ComponentType::UNSIGNED_SHORT);
        assert_eq!(p1.component_type, ComponentType::Other("OTHER".to_string()));
        assert_eq!(p2.component_type, ComponentType::UNSIGNED_SHORT);
        assert_eq!(p2.type_, ScaleOrVectorType::VEC3);
        assert_eq!(p3.type_, ScaleOrVectorType::Other("OTHER".to_string()));

        let json = r#"
        {
            "binaryBodyReference": {
                "byteOffset": 0,
                "componentType": "UNSIGNED_SHORT",
                "type": "SCALAR"
            },
            "property": []
        }"#;
        let batch_table: BatchTable = serde_json::from_str(json).unwrap();
        let binary_body_reference = batch_table.binary_body_reference.unwrap();
        let property = batch_table.property;
        assert_eq!(
            binary_body_reference,
            BinaryBodyReference {
                byte_offset: 0,
                component_type: ComponentType::UNSIGNED_SHORT,
                type_: ScaleOrVectorType::SCALAR,
            }
        );
        assert_eq!(property, Some(PropertySchema::Array(vec![])));

        let json = r#"
        {
        }"#;
        let batch_table: BatchTable = serde_json::from_str(json).unwrap();
        assert_eq!(batch_table.binary_body_reference, None);
        assert_eq!(batch_table.property, None);

        let json = r#"
        {
            "property": {
                "byteOffset": 0,
                "componentType": "UNSIGNED_SHORT",
                "type": "SCALAR"
            }
        }"#;
        let batch_table: BatchTable = serde_json::from_str(json).unwrap();
        let property =
            if let PropertySchema::BinaryBodyReference(property) = batch_table.property.unwrap() {
                property
            } else {
                panic!("property must be array")
            };
        assert_eq!(property.type_, ScaleOrVectorType::SCALAR);
        assert_eq!(property.component_type, ComponentType::UNSIGNED_SHORT);
        assert_eq!(property.byte_offset, 0);
    }
}
