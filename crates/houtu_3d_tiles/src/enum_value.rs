use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// An enum value.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValue {
    /// The name of the enum value.
    pub name: String,
    /// The description of the enum value.
    pub description: Option<String>,
    /// The integer enum value.
    pub value: i64,
}

impl ExtensibleObject for EnumValue {
    const TYPE_NAME: &'static str = "EnumValue";
}
