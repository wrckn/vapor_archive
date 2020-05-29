use crate::{
    shared::{
        compression::Compression,
        encryption::Encryption,
        file_metadata::FileMetadata
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
#[derive(Debug, Serialize, Deserialize)]
pub struct FileHeader {
    /// Byte range of this files' data
    pub data_range: Range<u64>, // 8 + 8 = 16B
    /// Byte size of the raw, uncompressed data in this file
    pub raw_size: u64,
    /// BLAKE2 checksum for the compressed data
    pub data_checksum: [u8; 32], // 16 + 32 = 48B
    /// BLAKE2 checksum for the uncompressed data,
    pub raw_checksum: [u8; 32], // 48 + 32 = 80B
    /// 8-bit integer ID of the compression algorithm used
    pub compression: u8, // 81B
    /// 8-bit integer ID of the encryption algorithm used
    pub encryption: u8, // 82B
    /// File Metadata
    pub metadata: FileMetadata
}

impl FileHeader {
    /// Returns a boolean indicating whether or not this file is compressed
    pub fn is_compressed(&self) -> bool {
        let compression_type = Compression::from(self.compression);
        match compression_type {
            Compression::None => false,
            _ => true
        }
    }

    /// Returns a boolean indicating whether or not this file is encrypted
    pub fn is_encrypted(&self) -> bool {
        let encryption_type = Encryption::from(self.encryption);
        match encryption_type {
            Encryption::None => false,
            _ => true
        }
    }
    
    /// Gets the compression type
    pub fn get_compression(&self) -> Compression {
        Compression::from(self.compression)
    }

    /// With a given raw size
    pub fn with_raw_size(mut self, raw_size: u64) -> Self {
        self.raw_size = raw_size;
        self
    }

    /// With a given compression type
    pub fn with_compression(mut self, compression_type: Compression) -> Self {
        self.compression = compression_type.into();
        self
    }

    /// With a given data range
    pub fn with_data_range(mut self, data_range: Range<u64>) -> Self {
        self.data_range = data_range;
        self
    }
    
    /// With a given data checksum
    pub fn with_data_checksum(mut self, data_checksum: [u8; 32]) -> Self {
        self.data_checksum = data_checksum;
        self
    }
    
    /// With a given raw checksum
    pub fn with_raw_checksum(mut self, raw_checksum: [u8; 32]) -> Self {
        self.raw_checksum = raw_checksum;
        self
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        Self {
            data_range: 0..0,
            data_checksum: [0; 32],
            raw_checksum: [0; 32],
            compression: 0,
            encryption: 0,
            raw_size: 0,
            metadata: FileMetadata::default()
        }
    }
}

