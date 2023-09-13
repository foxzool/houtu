use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

use crate::class_statistics::ClassStatistics;

/// Statistics about entities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    /// A dictionary, where each key corresponds to a class ID in the classes dictionary
    /// and each value is an object containing statistics about entities that conform to the class.
    pub classes: Option<HashMap<String, ClassStatistics>>,
}

impl ExtensibleObject for Statistics {
    const TYPE_NAME: &'static str = "Statistics";
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
                        count: 1,
                        properties: Some({
                            let mut map = HashMap::new();
                            map.insert(
                                "example".to_owned(),
                                crate::properties_statistics::PropertyStatistics {
                                    min: Some(crate::common::definitions::NumericValue::Numeric(
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
