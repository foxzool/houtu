use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::specification::common::RootProperty;

/// A 3D Tiles style.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Style {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// A dictionary object of `expression` strings mapped to a variable name key that may be referenced throughout the style. If an expression references a defined variable, it is replaced with the evaluated result of the corresponding expression.
    pub defines: Option<HashMap<String, Expression>>,
    /// A `boolean expression` or `conditions` property which determines if a feature should be shown.
    pub show: Option<OneOfShow>,
    /// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
    pub color: Option<OneOfColor>,
    /// A `meta` object which determines the values of non-visual properties of the feature.
    pub meta: Option<StyleMeta>,
}

/// A valid 3D Tiles style expression. Details are described in the 3D Tiles Styling specification.
pub type Expression = String;

/// 3D Tiles style `expression` that evaluates to a Color. Details are described in the 3D Tiles Styling specification.
pub type ColorExpression = String;

/// A `boolean expression` or `conditions` property which determines if a feature should be shown.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOfShow {
    BooleanExpression(BooleanExpression),
    Conditions(Conditions),
}

impl Default for OneOfShow {
    fn default() -> Self {
        Self::BooleanExpression(BooleanExpression::Boolean(true))
    }
}

/// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOfColor {
    Color(ColorExpression),
    Conditions(Conditions),
}

impl Default for OneOfColor {
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
    pub conditions: Vec<Condition>,
}

/// An `expression` evaluated as the result of a condition being true. An array of two expressions. If the first expression is evaluated and the result is `true`, then the second expression is evaluated and returned as the result of the condition.
pub type Condition = [Expression; 2];

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NumberExpression {
    Number(f64),
    String(String),
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StyleMeta {
    #[serde(flatten)]
    pub root: RootProperty,
    #[serde(flatten)]
    pub defines: HashMap<String, Expression>,
}

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
                conditions: vec![["true".to_owned(), "true".to_owned()]]
            }
        );

        let json = json!(
            {
                "conditions": []
            }
        );
        let conditions: Conditions = serde_json::from_value(json).unwrap();
        assert_eq!(
            conditions,
            Conditions {
                conditions: Vec::new()
            }
        );
    }

    #[test]
    fn test_one_of_show() {
        let json = json!(true);
        let show_properties: OneOfShow = serde_json::from_value(json).unwrap();
        assert_eq!(
            show_properties,
            OneOfShow::BooleanExpression(BooleanExpression::Boolean(true))
        );

        let json = json!("true".to_string());
        let show_properties: OneOfShow = serde_json::from_value(json).unwrap();
        assert_eq!(
            show_properties,
            OneOfShow::BooleanExpression(BooleanExpression::Expression("true".to_string()))
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
        let show_properties: OneOfShow = serde_json::from_value(json).unwrap();
        assert_eq!(
            show_properties,
            OneOfShow::Conditions(Conditions {
                conditions: vec![["true".to_owned(), "true".to_owned()]],
            })
        );

        let show_property = OneOfShow::default();
        assert_eq!(
            show_property,
            OneOfShow::BooleanExpression(BooleanExpression::Boolean(true))
        );
    }

    #[test]
    fn test_color_property() {
        let json = json!("#FFFFFF");
        let color_property: OneOfColor = serde_json::from_value(json).unwrap();
        assert_eq!(color_property, OneOfColor::Color("#FFFFFF".to_owned()));

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
        let color_property: OneOfColor = serde_json::from_value(json).unwrap();
        assert_eq!(
            color_property,
            OneOfColor::Conditions(Conditions {
                conditions: vec![["true".to_owned(), "true".to_owned()]],
            })
        );

        let color_property = OneOfColor::default();
        assert_eq!(color_property, OneOfColor::Color("#FFFFFF".to_owned()));
    }

    #[test]
    fn test_number_expression() {
        let json = json!(1.0);
        let number_expression: NumberExpression = serde_json::from_value(json).unwrap();
        assert_eq!(number_expression, NumberExpression::Number(1.0));

        let json = json!("1.0");
        let number_expression: NumberExpression = serde_json::from_value(json).unwrap();
        assert_eq!(
            number_expression,
            NumberExpression::String("1.0".to_owned())
        );
    }

    #[test]
    fn test_meta() {
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
                    "property": "value"
            }
        );
        let meta: StyleMeta = serde_json::from_value(json).unwrap();
        assert_eq!(
            meta.root.extensions.unwrap()["EXTENSION_NAME"]["property"],
            "value"
        );
        assert_eq!(meta.root.extras.unwrap()["property"], "value");
        assert_eq!(meta.defines["property"], "value");
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
                "meta": {
                    "property": "value"
                }

            }
        );
        let style: Style = serde_json::from_value(json).unwrap();
        let root = style.root;
        assert_eq!(
            root.extensions.unwrap()["EXTENSION_NAME"]["property"],
            "value"
        );
        assert_eq!(root.extras.unwrap()["property"], "value");
        assert_eq!(style.defines.unwrap()["example"], "true");
        assert_eq!(
            style.show.unwrap(),
            OneOfShow::BooleanExpression(BooleanExpression::Boolean(true))
        );
        assert_eq!(
            style.color.unwrap(),
            OneOfColor::Color("#FFFFFF".to_owned())
        );

        assert_eq!(style.meta.unwrap().defines["property"], "value");

        let json = json!({});
        let style: Style = serde_json::from_value(json).unwrap();
        assert_eq!(style.root, RootProperty::default());
        assert_eq!(style.defines, None);
        assert_eq!(style.show, None);
        assert_eq!(style.color, None);
        assert_eq!(style.meta, None);
    }
}
