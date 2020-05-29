use crate::{
    shared::{
        file_metadata::FileMetadata,
        compression::Compression,
        encryption::Encryption
    }
};

use std::{
    fmt::{
        Formatter,
        Result as FmtResult,
        Display
    }
};

use serde::{
    Serialize,
    Deserialize
};

/// FileInfo struct
/// 
/// Contains all relevant file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Byte size of the the uncompressed/decrypted data
    pub raw_size: u64,
    /// Byte size of the the compressed/encrypted data
    pub data_size: u64,
    /// The compression algorithm used
    pub compression: Compression,
    /// The encryption algorithm used
    pub encryption: Encryption,
    /// The filename
    pub filename: String,
    /// File metadata, such as permissions and timestamps
    pub metadata: FileMetadata,
    /// Checksum of the compressed/encrypted data
    pub data_checksum: [u8; 32],
    /// Checksum of the uncompressed/decrypted data
    pub raw_checksum: [u8; 32]
}

impl FileInfo {
    /// ...with a given filename
    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = String::from(filename);
        self
    }

    /// Returns a boolean indicating whether or not this file is compressed
    pub fn is_compressed(&self) -> bool {
        match self.compression {
            Compression::None => false,
            _ => true
        }
    }

    /// Returns a boolean indicating whether or not this file is encrypted
    pub fn is_encrypted(&self) -> bool {
        match self.encryption {
            Encryption::None => false,
            _ => true
        }
    }

    /// With a given raw size
    pub fn with_raw_size(mut self, raw_size: u64) -> Self {
        self.raw_size = raw_size;
        self
    }

    /// With a given data size
    pub fn with_data_size(mut self, data_size: u64) -> Self {
        self.data_size = data_size;
        self
    }

    /// With a given compression type
    pub fn with_compression(mut self, compression_type: Compression) -> Self {
        self.compression = compression_type;
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

impl Display for FileInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "FileInfo {{")?;
        writeln!(f, "    compression: {:?}", self.compression)?;
        writeln!(f, "    encryption: {:?}", self.encryption)?;
        write!(f, "    data_checksum: ")?;
        for byte in self.data_checksum.iter() {
            write!(f, "{:x}", byte)?;
        }
        writeln!(f, "")?;
        write!(f, "    raw_checksum: ")?;
        for byte in self.raw_checksum.iter() {
            write!(f, "{:x}", byte)?;
        }
        writeln!(f, "")?;
        writeln!(f, "}}")
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            data_checksum: [0; 32],
            raw_checksum: [0; 32],
            compression: Compression::None,
            encryption: Encryption::None,
            raw_size: 0,
            data_size: 0,
            filename: String::new(),
            metadata: FileMetadata::default()
        }
    }
}