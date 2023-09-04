use crate::common::{AnyValue, NoDataValue, NumericValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A class containing a set of properties.
#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// The name of the class, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the class.
    pub description: Option<String>,
    /// A dictionary, where each key is a property ID and each value is an object defining the property. Property IDs shall be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    pub properties: Option<HashMap<String, Property>>,
}

/// A single property of a metadata class.
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    /// The name of the property, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the property.
    pub description: Option<String>,
    /// The element type.
    #[serde(rename = "type")]
    pub element_type: ElementType,
    /// The datatype of the element's components. Required for `SCALAR`, `VECN`, and `MATN` types, and disallowed for other types.
    #[serde(rename = "componentType")]
    pub component_type: Option<ComponentType>,
    /// Enum ID as declared in the `enums` dictionary. Required when `type` is `ENUM`. Disallowed when `type` is not `ENUM`
    #[serde(rename = "enumType")]
    pub enum_type: Option<String>,
    /// Whether the property is an array. When `count` is defined the property is a fixed-length array. Otherwise the property is a variable-length array.
    pub array: Option<bool>,
    /// The number of elements in the array. Required when `array` is `true`.
    pub count: Option<usize>,
    /// Specifies whether integer values are normalized. Only applicable to `SCALAR`, `VECN`, and `MATN` types with integer component types. For unsigned integer component types, values are normalized between `[0.0, 1.0]`. For signed integer component types, values are normalized between `[-1.0, 1.0]`. For all other component types, this property shall be false.
    pub normalized: Option<bool>,
    /// An offset to apply to property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Not applicable to variable-length arrays.
    pub offset: Option<NumericValue>,
    /// A scale to apply to property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Not applicable to variable-length arrays.
    pub scale: Option<NumericValue>,
    /// Maximum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied. Not applicable to variable-length arrays.
    pub max: Option<NumericValue>,
    /// Minimum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied. Not applicable to variable-length arrays.
    pub min: Option<NumericValue>,
    /// If required, the property shall be present in every entity conforming to the class. If not required, individual entities may include `noData` values, or the entire property may be omitted. As a result, `noData` has no effect on a required property. Client implementations may use required properties to make performance optimizations.
    pub required: Option<bool>,
    /// A `noData` value represents missing data — also known as a sentinel value — wherever it appears. `BOOLEAN` properties may not specify `noData` values. This is given as the plain property value, without the transforms from the `normalized`, `offset`, and `scale` properties. Shall not be defined if `required` is true.
    #[serde(rename = "noData")]
    pub no_data: Option<NoDataValue>,
    /// A default value to use when encountering a `noData` value or an omitted property. The value is given in its final form, taking the effect of `normalized`, `offset`, and `scale` properties into account. Shall not be defined if `required` is true.
    pub default: Option<AnyValue>,
    /// An identifier that describes how this property should be interpreted. The semantic cannot be used by other properties in the class.
    pub semantic: Option<String>,
}

/// The element type.
#[derive(Debug, Serialize, Deserialize)]
pub enum ElementType {
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
    String(String),
}

/// The datatype of the element's components. Required for `SCALAR`, `VECN`, and `MATN` types, and disallowed for other types.
#[derive(Debug, Serialize, Deserialize)]
pub enum ComponentType {
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
    String(String),
}
