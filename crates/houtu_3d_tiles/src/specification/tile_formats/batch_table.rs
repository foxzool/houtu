use std::collections::HashMap;

use crate::specification::common::RootProperty;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::specification::tile_formats::feature_table::ComponentType;

/// A set of properties defining application-specific metadata for features in a tile.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTable {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// An object defining the reference to a section of the binary body of the batch table where the property values are stored if not defined directly in the JSON.
    pub binary_body_reference: Option<BinaryBodyReference>,
    /// A user-defined property which specifies per-feature application-specific metadata in a tile. Values either can be defined directly in the JSON as an array, or can refer to sections in the binary body with a `BinaryBodyReference` object.
    pub property: Option<Property>,
    #[serde(flatten)]
    pub additional_properties: HashMap<String, Property>,
}

/// An object defining the reference to a section of the binary body of the batch table where the property values are stored if not defined directly in the JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryBodyReference {
    pub byte_offset: u64,
    /// The datatype of components in the property.
    pub component_type: ComponentType,
    #[serde(rename = "type")]
    pub type_: BinaryBodyReferenceType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Property {
    BinaryBodyReference(BinaryBodyReference),
    Array(Vec<serde_json::Value>),
}

/// Specifies if the property is a scalar or vector.
#[derive(Debug, Serialize, Deserialize, PartialEq, EnumString)]
pub enum BinaryBodyReferenceType {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_property() {
        let json = json!([1, 2, 3]);
        let property: Property = serde_json::from_value(json).unwrap();

        assert_eq!(
            property,
            Property::Array(vec![json!(1), json!(2), json!(3)])
        );

        let json = json!({
            "byteOffset": 0,
            "componentType": "UNSIGNED_SHORT",
            "type": "SCALAR"
        });
        let property: Property = serde_json::from_value(json).unwrap();
        let binary_body_reference: BinaryBodyReference =
            if let Property::BinaryBodyReference(binary_body_reference) = property {
                binary_body_reference
            } else {
                panic!("property must be binary body reference")
            };

        assert_eq!(
            binary_body_reference,
            BinaryBodyReference {
                byte_offset: 0,
                component_type: ComponentType::UNSIGNED_SHORT,
                type_: BinaryBodyReferenceType::SCALAR,
            }
        );
    }

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
                    "componentType": "DOUBLE",
                    "type": "VEC3"
                },
                {
                    "byteOffset": 2,
                    "componentType": "UNSIGNED_SHORT",
                    "type": "VEC3"
                }
            ]
        }
        "#;

        let batch_table: BatchTable = serde_json::from_str(json).unwrap();
        assert!(batch_table.binary_body_reference.is_some());
        let binary_body_reference: BinaryBodyReference = batch_table.binary_body_reference.unwrap();
        assert!(batch_table.property.is_some());

        let property = if let Property::Array(property) = batch_table.property.unwrap() {
            property
        } else {
            panic!("property must be array")
        };
        assert_eq!(
            binary_body_reference.component_type,
            ComponentType::UNSIGNED_SHORT
        );
        assert_eq!(binary_body_reference.byte_offset, 0);
        assert_eq!(binary_body_reference.type_, BinaryBodyReferenceType::SCALAR);
        let p0: BinaryBodyReference = serde_json::from_value(property[0].clone()).unwrap();
        let p1: BinaryBodyReference = serde_json::from_value(property[1].clone()).unwrap();
        let p2: BinaryBodyReference = serde_json::from_value(property[2].clone()).unwrap();
        assert_eq!(p0.component_type, ComponentType::UNSIGNED_SHORT);
        assert_eq!(p1.component_type, ComponentType::DOUBLE);
        assert_eq!(p2.component_type, ComponentType::UNSIGNED_SHORT);
        assert_eq!(p2.type_, BinaryBodyReferenceType::VEC3);

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
                type_: BinaryBodyReferenceType::SCALAR,
            }
        );
        assert_eq!(property, Some(Property::Array(vec![])));

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
            if let Property::BinaryBodyReference(property) = batch_table.property.unwrap() {
                property
            } else {
                panic!("property must be array")
            };
        assert_eq!(property.type_, BinaryBodyReferenceType::SCALAR);
        assert_eq!(property.component_type, ComponentType::UNSIGNED_SHORT);
        assert_eq!(property.byte_offset, 0);
    }
}
