use crate::specification::style::{Conditions, NumberExpression, Style};
use serde::{Deserialize, Serialize};

/// A 3D Tiles style with additional properties for Point Clouds.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointCloudStyle {
    #[serde(flatten)]
    pub style: Style,
    /// A `number expression` or `conditions` property which determines the size of the points in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<OneOfPointSize>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOfPointSize {
    NumberExpression(NumberExpression),
    Conditions(Conditions),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_one_of_point_size() {
        let json = json!({
            "pointSize": 1.0
        });
        let point_cloud_style: PointCloudStyle = serde_json::from_value(json).unwrap();
        assert_eq!(
            point_cloud_style.point_size,
            Some(OneOfPointSize::NumberExpression(NumberExpression::Number(
                1.0
            )))
        );

        let json = json!({
            "pointSize": {
                "conditions": [
                    [
                        "e1",
                        "e2"
                    ]
                ]
            }
        });
        let point_cloud_style: PointCloudStyle = serde_json::from_value(json).unwrap();
        assert_eq!(
            point_cloud_style.point_size,
            Some(OneOfPointSize::Conditions(Conditions {
                conditions: vec![["e1".to_owned(), "e2".to_owned()]],
            }))
        );

        let json = json!({});
        let point_cloud_style: PointCloudStyle = serde_json::from_value(json).unwrap();
        assert_eq!(point_cloud_style.point_size, None);

        let point_cloud_style = PointCloudStyle {
            style: Default::default(),
            point_size: None,
        };
        assert_eq!(
            serde_json::to_value(point_cloud_style).unwrap(),
            json!({"color": null, "defines": null, "extensions": null, "extras": null, "meta": null, "show": null})
        );
        let point_cloud_style = PointCloudStyle {
            style: Default::default(),
            point_size: Some(OneOfPointSize::NumberExpression(NumberExpression::Number(
                1.0,
            ))),
        };
        assert_eq!(
            serde_json::to_value(point_cloud_style).unwrap(),
            json!({
                "color": null, "defines": null, "extensions": null, "extras": null, "meta": null, "show": null,
                "pointSize": 1.0
            })
        );
    }
}
