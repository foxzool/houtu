use serde::{Deserialize, Serialize};

/// An array of numeric values
#[derive(Debug, Serialize, Deserialize)]
pub struct NumericArray1D(Vec<serde_json::Number>);
/// An array of arrays of numeric values
#[derive(Debug, Serialize, Deserialize)]
pub struct NumericArray2D(Vec<Vec<serde_json::Number>>);

/// An array of boolean values
#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanArray1D(Vec<bool>);

/// An array of string values
#[derive(Debug, Serialize, Deserialize)]
pub struct StringArray1D(Vec<String>);

/// For `SCALAR` this is a number. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
#[derive(Debug, Serialize, Deserialize)]
pub enum NumericValue {
    Number(serde_json::Number),
    NumericArray1D(NumericArray1D),
    NumericArray2D(NumericArray2D),
}

/// For `SCALAR` this is a number. For `STRING` this is a string. For `ENUM` this is a string that shall be a valid enum `name`, not an integer value. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
#[derive(Debug, Serialize, Deserialize)]
pub enum NoDataValue {
    Number(serde_json::Number),
    String(String),
    NumericArray1D(NumericArray1D),
}

/// For `SCALAR` this is a number. For `STRING` this is a string. For `ENUM` this is a string that shall be a valid enum `name`, not an integer value. For `BOOLEAN` this is a boolean. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length array this is an array of `count` elements of the given `type`. For variable-length arrays this is an array of any length of the given `type`.
#[derive(Debug, Serialize, Deserialize)]
pub enum AnyValue {
    Number(serde_json::Number),
    NumericArray1D(NumericArray1D),
    NumericArray2D(NumericArray2D),
    String(String),
    StringArray1D(StringArray1D),
    Boolean(bool),
    BooleanArray1D(BooleanArray1D),
}

/// An object defining the values of an enum.
#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    /// The name of the enum, e.g. for display purposes.
    pub name: Option<String>,
    /// The description of the enum.
    pub description: Option<String>,
    #[serde(rename = "valueType")]
    pub value_type: EnumType,
    /// An array of enum values. Duplicate names or duplicate integer values are not allowed.
    pub values: Vec<EnumValue>,
}

/// The type of the integer enum value.
#[derive(Debug, Serialize, Deserialize)]
pub enum EnumType {
    INT8,
    UINT8,
    INT16,
    UINT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
    String(String),
}

/// An enum value.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumValue {
    /// The name of the enum value.The name of the enum value.
    pub name: String,
    /// The description of the enum value.
    pub description: Option<String>,
    /// The integer enum value.
    pub value: i64,
}
