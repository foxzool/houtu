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

#[derive(Debug, PartialEq)]
pub enum ValueType {
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for ValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "INT8" => Ok(ValueType::INT8),
            "UINT8" => Ok(ValueType::UINT8),
            "INT16" => Ok(ValueType::INT16),
            "UINT16" => Ok(ValueType::UINT16),
            "INT32" => Ok(ValueType::INT32),
            "UINT32" => Ok(ValueType::UINT32),
            "INT64" => Ok(ValueType::INT64),
            "UINT64" => Ok(ValueType::UINT64),
            _ => Ok(ValueType::Other(value)),
        }
    }
}

impl serde::Serialize for ValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ValueType::INT8 => serializer.serialize_str("INT8"),
            ValueType::UINT8 => serializer.serialize_str("UINT8"),
            ValueType::INT16 => serializer.serialize_str("INT16"),
            ValueType::UINT16 => serializer.serialize_str("UINT16"),
            ValueType::INT32 => serializer.serialize_str("INT32"),
            ValueType::UINT32 => serializer.serialize_str("UINT32"),
            ValueType::INT64 => serializer.serialize_str("INT64"),
            ValueType::UINT64 => serializer.serialize_str("UINT64"),
            ValueType::Other(value) => serializer.serialize_str(value),
        }
    }
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
        let json = json!("Other");
        let value_type: ValueType = serde_json::from_value(json).unwrap();
        assert_eq!(value_type, ValueType::Other("Other".to_string()));
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
