use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::specification::class_statistics::ClassStatistics;
use crate::specification::common::RootProperty;

/// Statistics about entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// A dictionary, where each key corresponds to a class ID in the classes dictionary
    /// and each value is an object containing statistics about entities that conform to the class.
    pub classes: Option<HashMap<String, ClassStatistics>>,
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_statistics() {
        let json = json!(
            {
                "classes": {
                    "example_CLASS": {
                        "count": 1,
                        "properties": {
                            "example": {
                                "min": 1,
                            }
                        }
                    }
                }
            }
        );
        let statistics: super::Statistics = serde_json::from_value(json).unwrap();
        assert_eq!(
            statistics.classes,
            Some({
                let mut map = HashMap::new();
                map.insert(
                    "example_CLASS".to_owned(),
                    super::ClassStatistics {
                        root: Default::default(),
                        count: 1,
                        properties: Some({
                            let mut map = HashMap::new();
                            map.insert(
                                "example".to_owned(),
                                crate::specification::properties_statistics::PropertyStatistics {
                                    min: Some(crate::specification::common::definitions::NumericValue::Numeric(
                                        1.0,
                                    )),
                                    ..Default::default()
                                },
                            );
                            map
                        }),
                    },
                );
                map
            })
        );
    }

    #[test]
    fn test_empty() {
        let json = json!({});
        let statistics: super::Statistics = serde_json::from_value(json).unwrap();
        assert_eq!(statistics.classes, None);
    }
}
