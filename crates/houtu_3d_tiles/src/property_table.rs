use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::property_table_property::PropertyTableProperty;

/// Properties conforming to a class, organized as property values stored in binary columnar arrays.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTable {
    /// The name of the property table, e.g. for display purposes.
    pub name: Option<String>,
    /// The class that property values conform to. The value shall be a class ID declared in the classes dictionary.
    #[serde(rename = "classProperty")]
    pub class_property: String,
    /// The number of elements in each property array.
    pub count: i64,
    /// A dictionary, where each key corresponds to a property ID in the class’ properties dictionary and each value is an object describing where property values are stored.
    /// Required properties shall be included in this dictionary.
    pub properties: Option<HashMap<String, PropertyTableProperty>>,
}

impl ExtensibleObject for PropertyTable {
    const TYPE_NAME: &'static str = "PropertyTable";
}
