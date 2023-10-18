use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::specification::common::definitions::AnyValue;
use crate::specification::common::RootProperty;

/// An object containing a reference to a class from a metadata schema,
/// and property values that conform to the properties of that class.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDataEntity {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The class that property values conform to. The value shall be a class ID declared in the `classes` dictionary of the metadata schema.
    pub class: String,
    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value contains the property values. The type of the value shall match the property definition: For `BOOLEAN` use `true` or `false`. For `STRING` use a JSON string. For numeric types use a JSON number. For `ENUM` use a valid enum `name`, not an integer value. For `ARRAY`, `VECN`, and `MATN` types use a JSON array containing values matching the `componentType`. Required properties shall be included in this dictionary.
    pub properties: HashMap<String, AnyValue>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_meta_data_entity() {
        let json = json!(
        {
            "class": "class",
            "properties": {
                "example_STRING": "string",
                "example_BOOLEAN": true,
                "example_ENUM": "enum",
                "example_ARRAY": [1, 2, 3],
                "example_VEC3": [1, 2, 3],
                "example_MAT4": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,11, 12, 13, 14, 15, 16]
            }
        });
        let meta_data_entity: super::MetaDataEntity = serde_json::from_value(json).unwrap();
        assert_eq!(meta_data_entity.class, "class");
        assert_eq!(meta_data_entity.properties.len(), 6);
    }
}
