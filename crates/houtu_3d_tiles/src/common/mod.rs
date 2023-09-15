pub mod definitions;

use serde::{Deserialize, Serialize};

/// Application-specific data.
pub type Extras = serde_json::Value;

/// Dictionary object with extension-specific objects.
pub type Extension = serde_json::Map<String, serde_json::Value>;
/// A basis for storing extensions and extras.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RootProperty {
    /// Dictionary object with extension-specific objects.
    pub extensions: Option<Extension>,
    /// Application-specific data.
    pub extras: Option<Extras>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_root_property() {
        let json = json!(
            {
                "extensions": {
                    "EXTENSION_NAME": {
                        "property": "value"
                    }
                },
                "extras": {
                    "property": "value"
                }
            }
        );
        let root_property: RootProperty = serde_json::from_value(json).unwrap();

        assert_eq!(
            root_property.extensions.unwrap()["EXTENSION_NAME"]["property"],
            "value"
        );
        assert_eq!(root_property.extras.unwrap()["property"], "value");

        let json = json!({});
        let root_property: RootProperty = serde_json::from_value(json).unwrap();
        assert_eq!(root_property.extensions, None);

        let json = json!({});
        let root_property: RootProperty = serde_json::from_value(json).unwrap();
        assert_eq!(root_property.extras, None);
    }
}
