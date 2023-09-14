use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{Extension, Extras};
use houtu_utility::ExtensibleObject;
pub use meta::*;

mod meta;

/// A 3D Tiles style.
#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    /// Dictionary object with extension-specific objects.
    pub extensions: Option<Extension>,
    /// Application-specific data.
    pub extras: Option<Extras>,
    /// A dictionary object of `expression` strings mapped to a variable name key that may be referenced throughout the style. If an expression references a defined variable, it is replaced with the evaluated result of the corresponding expression.
    pub defines: Option<HashMap<String, Expression>>,
    /// A `boolean expression` or `conditions` property which determines if a feature should be shown.
    pub show: Option<ShowProperty>,
    /// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
    pub color: Option<ColorProperty>,
    /// A `meta` object which determines the values of non-visual properties of the feature.
    #[serde(flatten)]
    pub meta: Option<HashMap<String, String>>,
}

impl ExtensibleObject for Style {
    const TYPE_NAME: &'static str = "Style";
}

/// A valid 3D Tiles style expression. Details are described in the 3D Tiles Styling specification.
pub type Expression = String;

/// 3D Tiles style `expression` that evaluates to a Color. Details are described in the 3D Tiles Styling specification.
pub type ColorExpression = String;

/// A `boolean expression` or `conditions` property which determines if a feature should be shown.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ShowProperty {
    Boolean(bool),
    Expression(Expression),
    Conditions(Conditions),
}

impl Default for ShowProperty {
    fn default() -> Self {
        Self::Boolean(true)
    }
}

/// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColorProperty {
    Color(ColorExpression),
    Conditions(Conditions),
}

impl Default for ColorProperty {
    fn default() -> Self {
        Self::Color("#FFFFFF".to_owned())
    }
}

/// A boolean or string with a 3D Tiles style expression that evaluates to a boolean. Details are described in the 3D Tiles Styling specification.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BooleanExpression {
    Boolean(bool),
    Expression(Expression),
}

/// A series of conditions evaluated in order, like a series of if...else statements that result in an expression being evaluated.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Conditions {
    /// A series of boolean conditions evaluated in order. For the first one that evaluates to true, its value, the 'result' (which is also an expression), is evaluated and returned. Result expressions shall all be the same type. If no condition evaluates to true, the result is `undefined`. When conditions is `undefined`, `null`, or an empty object, the result is `undefined`.
    pub conditions: Option<Vec<Condition>>,
}

/// An `expression` evaluated as the result of a condition being true. An array of two expressions. If the first expression is evaluated and the result is `true`, then the second expression is evaluated and returned as the result of the condition.
pub type Condition = [Expression; 2];

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_boolean_expression() {
        let json = json!(true);
        let boolean_expression: BooleanExpression = serde_json::from_value(json).unwrap();
        assert_eq!(boolean_expression, BooleanExpression::Boolean(true));

        let json = json!("true");
        let boolean_expression: BooleanExpression = serde_json::from_value(json).unwrap();
        assert_eq!(
            boolean_expression,
            BooleanExpression::Expression("true".to_owned())
        );
    }

    #[test]
    fn test_conditions() {
        let json = json!(
            {
                "conditions": [
                    [
                        "true",
                        "true"
                    ]
                ]
            }
        );
        let conditions: Conditions = serde_json::from_value(json).unwrap();

        assert_eq!(
            conditions,
            Conditions {
                conditions: Some(vec![["true".to_owned(), "true".to_owned()]]),
            }
        );

        let json = json!(
            {
                "conditions": null
            }
        );
        let conditions: Conditions = serde_json::from_value(json).unwrap();
        assert_eq!(conditions, Conditions { conditions: None });
    }

    #[test]
    fn test_show_properties() {
        let json = json!(true);
        let show_properties: ShowProperty = serde_json::from_value(json).unwrap();
        assert_eq!(show_properties, ShowProperty::Boolean(true));

        let json = json!("true".to_string());
        let show_properties: ShowProperty = serde_json::from_value(json).unwrap();
        assert_eq!(
            show_properties,
            ShowProperty::Expression("true".to_string())
        );

        let json = json!(
            {
                "conditions": [
                    [
                        "true",
                        "true"
                    ]
                ]
            }
        );
        let show_properties: ShowProperty = serde_json::from_value(json).unwrap();
        assert_eq!(
            show_properties,
            ShowProperty::Conditions(Conditions {
                conditions: Some(vec![["true".to_owned(), "true".to_owned()]]),
            })
        );

        let show_property = ShowProperty::default();
        assert_eq!(show_property, ShowProperty::Boolean(true));
    }

    #[test]
    fn test_color_property() {
        let json = json!("#FFFFFF");
        let color_property: ColorProperty = serde_json::from_value(json).unwrap();
        assert_eq!(color_property, ColorProperty::Color("#FFFFFF".to_owned()));

        let json = json!(
            {
                "conditions": [
                    [
                        "true",
                        "true"
                    ]
                ]
            }
        );
        let color_property: ColorProperty = serde_json::from_value(json).unwrap();
        assert_eq!(
            color_property,
            ColorProperty::Conditions(Conditions {
                conditions: Some(vec![["true".to_owned(), "true".to_owned()]]),
            })
        );

        let color_property = ColorProperty::default();
        assert_eq!(color_property, ColorProperty::Color("#FFFFFF".to_owned()));
    }

    #[test]
    fn test_style() {
        let json = json!(
            {
                "extensions": {
                    "EXTENSION_NAME": {
                        "property": "value"
                    }
                },
                "extras": {
                    "property": "value"
                },
                "defines": {
                    "example": "true"
                },
                "show": true,
                "color": "#FFFFFF",
                "property": "value"
            }
        );
        let style: Style = serde_json::from_value(json).unwrap();

        assert_eq!(
            style.extensions.unwrap()["EXTENSION_NAME"]["property"],
            "value"
        );
        assert_eq!(style.extras.unwrap()["property"], "value");
        assert_eq!(style.defines.unwrap()["example"], "true");
        assert_eq!(style.show.unwrap(), ShowProperty::Boolean(true));
        assert_eq!(
            style.color.unwrap(),
            ColorProperty::Color("#FFFFFF".to_owned())
        );
        assert_eq!(style.meta.unwrap()["property"], "value");

        let json = json!({});
        let style: Style = serde_json::from_value(json).unwrap();
        assert_eq!(style.extensions, None);
        assert_eq!(style.extras, None);
        assert_eq!(style.defines, None);
        assert_eq!(style.show, None);
        assert_eq!(style.color, None);
        assert_eq!(style.meta, Some(HashMap::new()));
    }
}
