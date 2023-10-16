use serde::{Deserialize, Serialize};

use crate::common::RootProperty;
use crate::enum_value::EnumValue;

/// An object defining the values of an enum.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Enum {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The name of the enum, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the enum.
    pub description: Option<String>,
    /// The type of the integer enum value.
    #[serde(rename = "valueType")]
    pub value_type: Option<ValueType>,
    /// An array of enum values. Duplicate names or duplicate integer values are not allowed.
    pub values: Vec<EnumValue>,
}

impl Default for Enum {
    fn default() -> Self {
        Self {
            root: Default::default(),
            name: None,
            description: None,
            value_type: Some(ValueType::UINT16),
            values: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_type() {
        let json = json!("INT8");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::INT8);
        let json = json!("UINT8");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::UINT8);
        let json = json!("INT16");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::INT16);
        let json = json!("UINT16");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::UINT16);
        let json = json!("INT32");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::INT32);
        let json = json!("UINT32");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::UINT32);
        let json = json!("INT64");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::INT64);
        let json = json!("UINT64");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::UINT64);
    }

    #[test]
    fn test_enum() {
        let json = json!(
            {
                "name": "name",
                "description": "description",
                "valueType": "UINT16",
                "values": [
                    {
                        "name": "name",
                        "description": "description",
                        "value": 1
                    }
                ]
            }
        );
        let enum_: Enum = serde_json::from_value(json).unwrap();
        assert_eq!(enum_.name, Some("name".to_owned()));
        assert_eq!(enum_.description, Some("description".to_owned()));
        assert_eq!(enum_.value_type, Some(ValueType::UINT16));
        assert_eq!(enum_.values.len(), 1);

        let json = json!(
            {
                "values": [
                    {
                        "name": "name",
                        "description": "description",
                        "value": 1
                    }
                ]
            }
        );
        let enum_: Enum = serde_json::from_value(json).unwrap();
        assert_eq!(enum_.name, None);
        assert_eq!(enum_.description, None);
        assert_eq!(enum_.value_type, None);
        assert_eq!(enum_.values.len(), 1);
    }
}
