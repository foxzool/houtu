use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// A single property of a metadata class.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClassProperty {
    /// The name of the property, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the property.
    pub description: Option<String>,
    /// The element type.
    #[serde(rename = "type")]
    pub type_: String,
    /// The datatype of the element's components. Required for `SCALAR`, `VECN`, and `MATN` types, and disallowed for other types.
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    /// Enum ID as declared in the `enums` dictionary. Required when `type` is `ENUM`. Disallowed when `type` is not `ENUM`
    #[serde(rename = "enumType")]
    pub enum_type: Option<String>,
    /// Whether the property is an array. When `count` is defined the property is a fixed-length array.
    /// Otherwise the property is a variable-length array.
    pub array: Option<bool>,
    /// The number of elements in the array. Required when `array` is `true`.
    pub count: Option<usize>,

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
pub mod type_ {
    pub const SCALAR: &str = "SCALAR";
    pub const VEC2: &str = "VEC2";
    pub const VEC3: &str = "VEC3";
    pub const VEC4: &str = "VEC4";
    pub const MAT2: &str = "MAT2";
    pub const MAT3: &str = "MAT3";
    pub const MAT4: &str = "MAT4";
    pub const STRING: &str = "STRING";
    pub const BOOLEAN: &str = "BOOLEAN";
    pub const ENUM: &str = "ENUM";
}

pub mod component_type {
    pub const INT8: &str = "INT8";
    pub const UINT8: &str = "UINT8";
    pub const INT16: &str = "INT16";
    pub const UINT16: &str = "UINT16";
    pub const INT32: &str = "INT32";
    pub const UINT32: &str = "UINT32";
    pub const INT64: &str = "INT64";
    pub const UINT64: &str = "UINT64";
    pub const FLOAT32: &str = "FLOAT32";
    pub const FLOAT64: &str = "FLOAT64";
}
