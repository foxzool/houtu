use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// An enum value.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_enum_value() {
        let json = json!({
            "name": "name",
            "description": "description",
            "value": 1
        });
        let enum_value: super::EnumValue = serde_json::from_value(json).unwrap();
        assert_eq!(enum_value.name, "name");
        assert_eq!(enum_value.description, Some("description".to_owned()));
        assert_eq!(enum_value.value, 1);
    }
}
