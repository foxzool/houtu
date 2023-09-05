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
pub mod value_type {
    pub const INT8: &str = "INT8";
    pub const UINT8: &str = "UINT8";
    pub const INT16: &str = "INT16";
    pub const UINT16: &str = "UINT16";
    pub const INT32: &str = "INT32";
    pub const UINT32: &str = "UINT32";
    pub const INT64: &str = "INT64";
    pub const UINT64: &str = "UINT64";
}
