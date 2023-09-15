use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::class::Class;
use crate::common::RootProperty;
use crate::enum_::Enum;

/// An object defining classes and enums.
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// Unique identifier for the schema.
    /// Schema IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub id: String,
    /// The name of the schema, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the schema.
    pub description: Option<String>,
    /// Application-specific version of the schema.
    pub version: Option<String>,
    ///A dictionary, where each key is a class ID and each value is an object defining the class. Class IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub classes: Option<HashMap<String, Class>>,
    /// A dictionary, where each key is an enum ID and each value is an object defining the values for the enum. Enum IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub enums: Option<HashMap<String, Enum>>,
}

impl ExtensibleObject for Schema {
    const TYPE_NAME: &'static str = "Schema";
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_schema() {
        let json = json!(
            {
                "id": "id",
                "name": "name",
                "description": "description",
                "version": "version",
                "classes": {
                    "example_CLASS": {
                        "name": "name",
                        "description": "description",
                        "properties": {
                            "example_STRING": {
                                "name": "name",
                                "description": "description",
                                "semantic": "semantic",
                                "type": "STRING"
                            }
                        }
                    }
                },
                "enums": {
                    "example_ENUM": {
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
                }
            }
        );
        let schema: Schema = serde_json::from_value(json).unwrap();
        assert_eq!(schema.id, "id");
        assert_eq!(schema.name, Some("name".to_owned()));
        assert_eq!(schema.description, Some("description".to_owned()));
        assert_eq!(schema.version, Some("version".to_owned()));
        assert_eq!(schema.classes.unwrap().len(), 1);
        assert_eq!(schema.enums.unwrap().len(), 1);

        let json = json!(
            {
                "id": "id",
            }
        );
        let schema: Schema = serde_json::from_value(json).unwrap();
        assert_eq!(schema.id, "id");
        assert_eq!(schema.name, None);
        assert_eq!(schema.description, None);
        assert_eq!(schema.version, None);
        assert_eq!(schema.classes, None);
        assert_eq!(schema.enums, None);
    }
}
