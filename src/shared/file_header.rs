use crate::{
    shared::{
        file_info::FileInfo
    }
};

use std::{
    ops::Range
};

use serde::{
    Serialize,
    Deserialize
};

/// A file header
/// 
/// Currently only contains the byte range, compression type and checksum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHeader {
    /// Byte range of this files' data
    pub data_range: Range<u64>, // 8 + 8 = 16B
    /// File information
    pub file_info: FileInfo
}

impl FileHeader {
    /// ...with a given file info struct
    pub fn with_file_info(mut self, file_info: FileInfo) -> Self {
        self.file_info = file_info;
        self
    }
    
    /// ...with a given data range
    pub fn with_data_range(mut self, data_range: Range<u64>) -> Self {
        self.data_range = data_range;
        self
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        Self {
            data_range: 0..0,
            file_info: FileInfo::default()
        }
    }
}

