use crate::specification::common::RootProperty;
use serde::{Deserialize, Serialize};

/// A dictionary object of metadata about per-feature properties.
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The maximum value of this property of all the features in the tileset.
    pub maximum: f64,
    /// The minimum value of this property of all the features in the tileset.
    pub minimum: f64,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_properties() {
        let json = json!({
            "maximum": 1.0,
            "minimum": 0.0
        });
        let properties: super::Properties = serde_json::from_value(json).unwrap();
        assert_eq!(properties.maximum, 1.0);
        assert_eq!(properties.minimum, 0.0);
    }
}
