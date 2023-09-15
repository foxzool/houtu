use serde::{Deserialize, Serialize};

use crate::common::definitions::{AnyValue, NoDataValue, NumericValue};
use crate::common::RootProperty;

/// A single property of a metadata class.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ClassProperty {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// The name of the property, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the property.
    pub description: Option<String>,
    /// The element type.
    #[serde(rename = "type")]
    pub type_: ElementType,
    /// The datatype of the element's components. Required for `SCALAR`, `VECN`, and `MATN` types, and disallowed for other types.
    #[serde(rename = "componentType")]
    pub component_type: Option<ComponentType>,
    /// Enum ID as declared in the `enums` dictionary. Required when `type` is `ENUM`. Disallowed when `type` is not `ENUM`
    #[serde(rename = "enumType")]
    pub enum_type: Option<String>,
    /// Whether the property is an array. When `count` is defined the property is a fixed-length array.
    /// Otherwise the property is a variable-length array.
    pub array: Option<bool>,
    /// The number of elements in the array. Required when `array` is `true`.
    pub count: Option<usize>,
    /// Specifies whether integer values are normalized.
    /// Only applicable to SCALAR, VECN, and MATN types with integer component types.
    /// For unsigned integer component types, values are normalized between [0.0, 1.0].
    /// For signed integer component types, values are normalized between [-1.0, 1.0].
    /// For all other component types, this property shall be false.
    pub normalized: Option<bool>,
    /// An offset to apply to property values.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`,
    /// or when the property is `normalized`. Not applicable to variable-length arrays.
    pub offset: Option<NumericValue>,
    /// A scale to apply to property values.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`,
    /// or when the property is `normalized`. Not applicable to variable-length arrays.
    pub scale: Option<NumericValue>,
    /// Maximum allowed value for the property.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the maximum of all property values, after the transforms
    /// based on the `normalized`, `offset`, and `scale` properties have been applied.
    /// Not applicable to variable-length arrays.
    pub max: Option<NumericValue>,
    /// Minimum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the minimum of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    /// Not applicable to variable-length arrays.
    pub min: Option<NumericValue>,
    /// If required, the property shall be present in every entity conforming to the class.
    /// If not required, individual entities may include `noData` values,
    /// or the entire property may be omitted. As a result, `noData` has no effect on a required property.
    /// Client implementations may use required properties to make performance optimizations.
    pub required: Option<bool>,
    /// A `noData` value represents missing data — also known as a sentinel value — wherever it appears.
    /// `BOOLEAN` properties may not specify `noData` values.
    /// This is given as the plain property value,
    /// without the transforms from the `normalized`, `offset`, and `scale` properties.
    /// Shall not be defined if `required` is true.
    #[serde(rename = "noData")]
    pub no_data: Option<NoDataValue>,
    /// A default value to use when encountering a `noData` value or an omitted property.
    /// The value is given in its final form,
    /// taking the effect of `normalized`, `offset`, and `scale` properties into account.
    /// Shall not be defined if `required` is true.
    #[serde(rename = "default")]
    pub default: Option<AnyValue>,
    /// An identifier that describes how this property should be interpreted.
    /// The semantic cannot be used by other properties in the class.
    pub semantic: Option<String>,
}

/// The element type.
#[derive(Debug, Default, PartialEq)]
pub enum ElementType {
    #[default]
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    MAT2,
    MAT3,
    MAT4,
    STRING,
    BOOLEAN,
    ENUM,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for ElementType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SCALAR" => Ok(ElementType::SCALAR),
            "VEC2" => Ok(ElementType::VEC2),
            "VEC3" => Ok(ElementType::VEC3),
            "VEC4" => Ok(ElementType::VEC4),
            "MAT2" => Ok(ElementType::MAT2),
            "MAT3" => Ok(ElementType::MAT3),
            "MAT4" => Ok(ElementType::MAT4),
            "STRING" => Ok(ElementType::STRING),
            "BOOLEAN" => Ok(ElementType::BOOLEAN),
            "ENUM" => Ok(ElementType::ENUM),
            _ => Ok(ElementType::Other(value)),
        }
    }
}

impl serde::Serialize for ElementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ElementType::SCALAR => serializer.serialize_str("SCALAR"),
            ElementType::VEC2 => serializer.serialize_str("VEC2"),
            ElementType::VEC3 => serializer.serialize_str("VEC3"),
            ElementType::VEC4 => serializer.serialize_str("VEC4"),
            ElementType::MAT2 => serializer.serialize_str("MAT2"),
            ElementType::MAT3 => serializer.serialize_str("MAT3"),
            ElementType::MAT4 => serializer.serialize_str("MAT4"),
            ElementType::STRING => serializer.serialize_str("STRING"),
            ElementType::BOOLEAN => serializer.serialize_str("BOOLEAN"),
            ElementType::ENUM => serializer.serialize_str("ENUM"),
            ElementType::Other(value) => serializer.serialize_str(value),
        }
    }
}

/// The datatype of the element's components. Only applicable to `SCALAR`, `VECN`, and `MATN` types.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum ComponentType {
    #[default]
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
    FLOAT32,
    FLOAT64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_element_type() {
        let json = json!("SCALAR");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::SCALAR);

        let json = json!("VEC2");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::VEC2);

        let json = json!("VEC3");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::VEC3);

        let json = json!("VEC4");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::VEC4);

        let json = json!("MAT2");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::MAT2);

        let json = json!("MAT3");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::MAT3);

        let json = json!("MAT4");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::MAT4);

        let json = json!("STRING");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::STRING);

        let json = json!("BOOLEAN");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::BOOLEAN);

        let json = json!("ENUM");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::ENUM);

        let json = json!("Other");
        let element_type: ElementType = serde_json::from_value(json).unwrap();
        assert_eq!(element_type, ElementType::Other("Other".to_owned()));
    }

    #[test]
    fn test_component_type() {
        let component_type = ComponentType::default();
        assert_eq!(component_type, ComponentType::INT8);

        let json = json!("INT8");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::INT8);

        let json = json!("UINT8");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::UINT8);

        let json = json!("INT16");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::INT16);

        let json = json!("UINT16");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::UINT16);

        let json = json!("INT32");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::INT32);

        let json = json!("UINT32");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::UINT32);

        let json = json!("INT64");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::INT64);

        let json = json!("UINT64");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::UINT64);

        let json = json!("FLOAT32");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::FLOAT32);

        let json = json!("FLOAT64");
        let component_type: ComponentType = serde_json::from_value(json).unwrap();
        assert_eq!(component_type, ComponentType::FLOAT64);

        let json = json!("Other");
        assert!(serde_json::from_value::<ComponentType>(json).is_err())
    }

    #[test]
    fn test_class_property() {
        let json = json!(
            {
                "name": "name",
                "description": "description",
                "type": "STRING",
                "componentType": "FLOAT32",
                "enumType": "enumType",
                "array": true,
                "count": 1,
                "normalized": true,
                "offset": 1,
                "scale": 1,
                "max": 1,
                "min": 1,
                "required": true,
                "noData": 1,
                "default": 1,
                "semantic": "semantic"
            }
        );
        let class_property: ClassProperty = serde_json::from_value(json).unwrap();
        assert_eq!(class_property.name, Some("name".to_owned()));
        assert_eq!(class_property.description, Some("description".to_owned()));
        assert_eq!(class_property.type_, ElementType::STRING);
        assert_eq!(class_property.component_type, Some(ComponentType::FLOAT32));
        assert_eq!(class_property.enum_type, Some("enumType".to_owned()));
        assert_eq!(class_property.array, Some(true));
        assert_eq!(class_property.count, Some(1));
        assert_eq!(class_property.normalized, Some(true));
        assert_eq!(class_property.offset, Some(NumericValue::Numeric(1.0)));
        assert_eq!(class_property.scale, Some(NumericValue::Numeric(1.0)));
        assert_eq!(class_property.max, Some(NumericValue::Numeric(1.0)));
        assert_eq!(class_property.min, Some(NumericValue::Numeric(1.0)));
        assert_eq!(class_property.required, Some(true));
        assert_eq!(class_property.no_data, Some(NoDataValue::Numeric(1.0)));
        assert_eq!(class_property.default, Some(AnyValue::Numeric(1.0)));
        assert_eq!(class_property.semantic, Some("semantic".to_owned()));

        let json = json!({
            "type": "SCALAR",
        });
        let class_property: ClassProperty = serde_json::from_value(json).unwrap();
        assert_eq!(class_property.name, None);
        assert_eq!(class_property.description, None);
        assert_eq!(class_property.type_, ElementType::SCALAR);
        assert_eq!(class_property.component_type, None);
        assert_eq!(class_property.enum_type, None);
        assert_eq!(class_property.array, None);
        assert_eq!(class_property.count, None);
        assert_eq!(class_property.normalized, None);
        assert_eq!(class_property.offset, None);
        assert_eq!(class_property.scale, None);
        assert_eq!(class_property.max, None);
        assert_eq!(class_property.min, None);
        assert_eq!(class_property.required, None);
        assert_eq!(class_property.no_data, None);
        assert_eq!(class_property.default, None);
        assert_eq!(class_property.semantic, None);
    }
}
