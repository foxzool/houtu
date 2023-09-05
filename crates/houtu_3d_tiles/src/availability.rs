use serde::{Deserialize, Serialize};

use houtu_utility::ExtensibleObject;

/// An object describing the availability of a set of elements.
#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    /// Index of a buffer view that indicates whether each element is available.
    /// The bitstream conforms to the boolean array encoding described in the 3D Metadata specification.
    /// If an element is available, its bit is 1, and if it is unavailable, its bit is 0.
    pub bitstream: Option<i64>,
    /// A number indicating how many 1 bits exist in the availability bitstream.
    #[serde(rename = "availableCount")]
    pub available_count: Option<i64>,
    /// Integer indicating whether all of the elements are available (1) or all are unavailable (0).
    pub constant: Option<Constant>,
}

impl ExtensibleObject for Availability {
    const TYPE_NAME: &'static str = "Availability";
}

#[derive(Debug)]
pub enum Constant {
    AVAILABLE,
    UNAVAILABLE,
    Other(i32),
}

impl<'de> serde::Deserialize<'de> for Constant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(Constant::UNAVAILABLE),
            1 => Ok(Constant::AVAILABLE),
            _ => Ok(Constant::Other(value)),
        }
    }
}

impl serde::Serialize for Constant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Constant::AVAILABLE => serializer.serialize_i32(1),
            Constant::UNAVAILABLE => serializer.serialize_i32(0),
            Constant::Other(value) => serializer.serialize_i32(*value),
        }
    }
}
