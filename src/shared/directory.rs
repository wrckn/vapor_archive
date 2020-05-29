use std::{
    ops::Range,
    collections::HashMap
};

use serde::{
    Serialize,
    Deserialize
};

/// Main directory struct.
/// 
/// Maps filenames to a byte range corresponding to the respective file header
#[derive(Serialize, Deserialize)]
pub struct Directory {
    /// Byte ranges of the file headers by filename
    pub file_headers: HashMap<String, Range<u64>>
}

impl Directory {
    /// Get a files header byte range by filename
    pub fn get_file(&self, path: &str) -> Option<Range<u64>> {
        self.file_headers.get(path).cloned()
    }
    
    /// Gets a list of all files
    pub fn get_file_list(&self) -> Vec<String> {
        self.file_headers.keys().cloned().collect()
    }

    /// Sets a file range
    pub fn set_file(&mut self, path: &str, header_range: Range<u64>) {
        self.file_headers.insert(String::from(path), header_range);
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            file_headers: HashMap::new()
        }
    }
}