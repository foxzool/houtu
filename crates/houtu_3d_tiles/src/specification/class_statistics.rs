use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::specification::common::RootProperty;

use crate::specification::properties_statistics::PropertyStatistics;

/// Statistics about entities that conform to a class.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassStatistics {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The number of entities that conform to the class.
    pub count: u64,
    /// A dictionary, where each key corresponds to a property ID in the class’ properties dictionary and each value is an object containing statistics about property values.
    pub properties: Option<HashMap<String, PropertyStatistics>>,
}

#[cfg(test)]
mod tests {
    use crate::specification::common::definitions::NumericValue;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_class_statistics() {
        let json = json!(
            {
                "count": 1,
                "properties": {
                    "example": {
                        "min": 1,
                    }
                }
            }
        );
        let class_statistics: ClassStatistics = serde_json::from_value(json).unwrap();
        assert_eq!(class_statistics.count, 1);
        assert_eq!(
            class_statistics.properties,
            Some({
                let mut map = HashMap::new();
                map.insert(
                    "example".to_owned(),
                    PropertyStatistics {
                        min: Some(NumericValue::Numeric(1.0)),
                        ..Default::default()
                    },
                );
                map
            })
        );
    }
}
