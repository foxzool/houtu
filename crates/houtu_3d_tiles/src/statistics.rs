use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::class_statistics::ClassStatistics;

/// Statistics about entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    /// A dictionary, where each key corresponds to a class ID in the classes dictionary and each value is an object containing statistics about entities that conform to the class.
    pub classes: HashMap<String, ClassStatistics>,
}

impl ExtensibleObject for Statistics {
    const TYPE_NAME: &'static str = "Statistics";
}
