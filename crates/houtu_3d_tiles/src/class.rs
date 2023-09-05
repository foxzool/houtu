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
