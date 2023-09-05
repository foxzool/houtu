use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::properties_statistics::PropertyStatistics;

/// Statistics about entities that conform to a class.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassStatistics {
    /// The number of entities that conform to the class.
    pub count: i64,
    /// A dictionary, where each key corresponds to a property ID in the class’ properties dictionary and each value is an object containing statistics about property values.
    pub properties: Option<HashMap<String, PropertyStatistics>>,
}

impl ExtensibleObject for ClassStatistics {
    const TYPE_NAME: &'static str = "ClassStatistics";
}
