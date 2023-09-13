use crate::class_property::ClassProperty;
use houtu_utility::ExtensibleObject;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A class containing a set of properties.
#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// The name of the class, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the class.
    pub description: Option<String>,
    /// A dictionary, where each key is a property ID and each value is an object defining the property. Property IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub properties: Option<HashMap<String, ClassProperty>>,
}

impl ExtensibleObject for Class {
    const TYPE_NAME: &'static str = "Class";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_property::ElementType;
    use serde_json::json;

    #[test]
    fn test_class() {
        let json = json!(
            {
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
        );
        let class: Class = serde_json::from_value(json).unwrap();
        assert_eq!(class.name, Some("name".to_owned()));
        assert_eq!(class.description, Some("description".to_owned()));
        assert_eq!(
            class.properties,
            Some({
                let mut map = HashMap::new();
                map.insert(
                    "example_STRING".to_owned(),
                    ClassProperty {
                        name: Some("name".to_owned()),
                        description: Some("description".to_owned()),
                        semantic: Some("semantic".to_owned()),
                        type_: ElementType::STRING,
                        ..Default::default()
                    },
                );
                map
            })
        );
    }
}
