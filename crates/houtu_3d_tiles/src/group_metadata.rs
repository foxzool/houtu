use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// An object containing metadata about a group.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMetadata {
    /// The class that property values conform to.
    /// The value shall be a class ID declared in the classes dictionary of the metadata schema.
    pub class: String,
    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary
    /// and each value contains the property values. The type of the value shall
    /// match the property definition: For `BOOLEAN` use `true` or `false`.
    /// For `STRING` use a JSON string. For numeric types use a JSON number.
    /// For `ENUM` use a valid enum `name`, not an integer value.
    /// For `ARRAY`, `VECN`, and `MATN` types use a JSON array containing values matching the `componentType`.
    /// Required properties shall be included in this dictionary.
    pub properties: HashMap<String, serde_json::Value>,
}

impl ExtensibleObject for GroupMetadata {
    const TYPE_NAME: &'static str = "GroupMetadata";
}
