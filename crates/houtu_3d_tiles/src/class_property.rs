use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// A single property of a metadata class.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ClassProperty {
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
    pub offset: Option<serde_json::Value>,
    /// A scale to apply to property values.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`,
    /// or when the property is `normalized`. Not applicable to variable-length arrays.
    pub scale: Option<serde_json::Value>,
    /// Maximum allowed value for the property.
    /// Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the maximum of all property values, after the transforms
    /// based on the `normalized`, `offset`, and `scale` properties have been applied.
    /// Not applicable to variable-length arrays.
    pub max: Option<serde_json::Value>,
    /// Minimum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types.
    /// This is the minimum of all property values,
    /// after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    /// Not applicable to variable-length arrays.
    pub min: Option<serde_json::Value>,
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
    pub no_data: Option<serde_json::Value>,
    /// A default value to use when encountering a `noData` value or an omitted property.
    /// The value is given in its final form,
    /// taking the effect of `normalized`, `offset`, and `scale` properties into account.
    /// Shall not be defined if `required` is true.
    #[serde(rename = "default")]
    pub default_property: Option<serde_json::Value>,
    /// An identifier that describes how this property should be interpreted.
    /// The semantic cannot be used by other properties in the class.
    pub semantic: Option<String>,
}

impl ExtensibleObject for ClassProperty {
    const TYPE_NAME: &'static str = "ClassProperty";
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
#[derive(Debug, Default, PartialEq)]
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
    Other(String),
}

impl<'de> serde::Deserialize<'de> for ComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "INT8" => Ok(ComponentType::INT8),
            "UINT8" => Ok(ComponentType::UINT8),
            "INT16" => Ok(ComponentType::INT16),
            "UINT16" => Ok(ComponentType::UINT16),
            "INT32" => Ok(ComponentType::INT32),
            "UINT32" => Ok(ComponentType::UINT32),
            "INT64" => Ok(ComponentType::INT64),
            "UINT64" => Ok(ComponentType::UINT64),
            "FLOAT32" => Ok(ComponentType::FLOAT32),
            "FLOAT64" => Ok(ComponentType::FLOAT64),
            _ => Ok(ComponentType::Other(value)),
        }
    }
}

impl serde::Serialize for ComponentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ComponentType::INT8 => serializer.serialize_str("INT8"),
            ComponentType::UINT8 => serializer.serialize_str("UINT8"),
            ComponentType::INT16 => serializer.serialize_str("INT16"),
            ComponentType::UINT16 => serializer.serialize_str("UINT16"),
            ComponentType::INT32 => serializer.serialize_str("INT32"),
            ComponentType::UINT32 => serializer.serialize_str("UINT32"),
            ComponentType::INT64 => serializer.serialize_str("INT64"),
            ComponentType::UINT64 => serializer.serialize_str("UINT64"),
            ComponentType::FLOAT32 => serializer.serialize_str("FLOAT32"),
            ComponentType::FLOAT64 => serializer.serialize_str("FLOAT64"),
            ComponentType::Other(value) => serializer.serialize_str(value),
        }
    }
}
