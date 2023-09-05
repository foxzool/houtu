use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// An array of binary property values.
/// This represents one column of a property table, and contains one value of a certain property for each metadata entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTableProperty {
    /// The index of the buffer view containing property values.
    /// The data type of property values is determined by the property definition:
    /// When type is BOOLEAN values are packed into a bitstream.
    /// When type is STRING values are stored as byte sequences and decoded as UTF-8 strings.
    /// When type is SCALAR, VECN, or MATN the values are stored as the provided componentType
    /// and the buffer view byteOffset shall be aligned to a multiple of the componentType size.
    /// When type is ENUM values are stored as the enum’s valueType and the buffer view byteOffset
    /// shall be aligned to a multiple of the valueType size.
    /// Each enum value in the array shall match one of the allowed values in the enum definition.
    /// arrayOffsets is required for variable-length arrays
    /// and stringOffsets is required for strings (for variable-length arrays of strings, both are required).
    pub values: i64,
    #[serde(rename = "arrayOffsets")]
    /// The index of the buffer view containing offsets for variable-length arrays.
    /// The number of offsets is equal to the property table count plus one.
    /// The offsets represent the start positions of each array,
    /// with the last offset representing the position after the last array.
    /// The array length is computed using the difference between the subsequent offset and the current offset.
    /// If type is STRING the offsets index into the string offsets array (stored in stringOffsets),
    /// otherwise they index into the property array (stored in values).
    /// The data type of these offsets is determined by arrayOffsetType.
    /// The buffer view byteOffset shall be aligned to a multiple of the arrayOffsetType size.
    pub array_offsets: Option<i64>,
    /// The index of the buffer view containing offsets for strings.
    /// The number of offsets is equal to the number of string elements plus one.
    /// The offsets represent the byte offsets of each string in the property array (stored in values),
    /// with the last offset representing the byte offset after the last string.
    /// The string byte length is computed using the difference between the subsequent offset and the current offset.
    /// The data type of these offsets is determined by stringOffsetType.
    /// The buffer view byteOffset shall be aligned to a multiple of the stringOffsetType size.
    #[serde(rename = "stringOffsets")]
    pub string_offsets: Option<i64>,
    /// The type of values in arrayOffsets.
    #[serde(rename = "arrayOffsetType")]
    pub array_offset_type: ArrayOffsetType,
    /// The type of values in stringOffsets.
    #[serde(rename = "stringOffsetType")]
    pub string_offset_type: StringOffsetType,
    /// An offset to apply to property values. Only applicable when the component type is FLOAT32 or FLOAT64, or when the property is normalized.
    /// Overrides the class property’s offset if both are defined.
    pub offset: Option<serde_json::Value>,
    /// A scale to apply to property values. Only applicable when the component type is FLOAT32 or FLOAT64, or when the property is normalized.
    /// Overrides the class property’s scale if both are defined.
    pub scale: Option<serde_json::Value>,
    /// Maximum value present in the property values. Only applicable to SCALAR, VECN, and MATN types.
    /// This is the maximum of all property values, after the transforms based on the normalized, offset,
    /// and scale properties have been applied.
    pub max: Option<serde_json::Value>,
    /// Minimum value present in the property values. Only applicable to SCALAR, VECN, and MATN types.
    /// This is the minimum of all property values, after the transforms based on the normalized, offset,
    /// and scale properties have been applied.
    pub min: Option<serde_json::Value>,
}

/// Known values for The type of values in `arrayOffsets`.
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString)]
pub enum ArrayOffsetType {
    UINT8,
    UINT16,
    UINT32,
    UINT64,
}

/// Known values for The type of values in `stringOffsets`.
#[derive(Debug, Serialize, Deserialize, strum_macros::EnumString)]
pub enum StringOffsetType {
    UINT8,
    UINT16,
    UINT32,
    UINT64,
}

impl ExtensibleObject for PropertyTableProperty {
    const TYPE_NAME: &'static str = "PropertyTableProperty";
}
