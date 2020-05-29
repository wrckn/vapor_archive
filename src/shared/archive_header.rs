use std::{
    ops::Range
};

use serde::{
    Serialize,
    Deserialize
};
/// The archive header
/// 
/// Contains archive metadata and points to the most up-to-date directory.
#[derive(Serialize, Deserialize, Debug)]
pub struct ArchiveHeader {
    /// The library version used to create the archive
    /// 
    /// Format: (MAJOR, MINOR, PATCH)
    pub version: (u8, u8, u8),
    /// Byte range that points to the most up-to-date directory
    pub directory: Range<u64>
}

impl Default for ArchiveHeader {
    fn default() -> Self {
        Self {
            version: crate::get_version_tuple(),
            directory: 0..0
        }
    }
}

