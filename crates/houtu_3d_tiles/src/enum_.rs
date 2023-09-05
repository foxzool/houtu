use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// An object defining the values of an enum.
#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    /// The name of the enum, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the enum.
    pub description: Option<String>,
    /// The type of the integer enum value.
    #[serde(rename = "valueType")]
    pub value_type: String,
}

impl ExtensibleObject for Enum {
    const TYPE_NAME: &'static str = "Enum";
}

impl Default for Enum {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            value_type: "UINT16".to_string(),
        }
    }
}

#[allow(dead_code)]
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
