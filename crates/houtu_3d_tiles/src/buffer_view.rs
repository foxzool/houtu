use houtu_utility::ExtensibleObject;
use serde::{Deserialize, Serialize};

/// A contiguous subset of a buffer
#[derive(Debug, Serialize, Deserialize)]
pub struct BufferView {
    /// The index of the buffer.
    pub buffer: i64,
    /// The offset into the buffer in bytes.
    pub byte_offset: i64,
    /// The total byte length of the buffer view.
    pub byte_length: i64,
    /// The name of the bufferView.
    pub name: Option<String>,
}

impl ExtensibleObject for BufferView {
    const TYPE_NAME: &'static str = "BufferView";
}
