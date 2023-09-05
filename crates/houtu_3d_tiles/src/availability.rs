use houtu_utility::ExtensibleObject;
use serde::{Deserialize, Serialize};

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
    pub constant: Option<i32>,
}

impl ExtensibleObject for Availability {
    const TYPE_NAME: &'static str = "Availability";
}

pub struct Constant;

#[allow(dead_code)]
impl Constant {
    const UNAVAILABLE: i32 = 0;
    const AVAILABLE: i32 = 1;
}
