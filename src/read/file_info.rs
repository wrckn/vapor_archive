use crate::{
    shared::{
        file_header::FileHeader,
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

pub struct FileInfo {
    pub compression: Compression,
    pub encryption: Encryption,
    pub filename: String,
    pub data_checksum: [u8; 32],
    pub raw_checksum: [u8; 32]
}

impl FileInfo {
    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = String::from(filename);
        self
    }
}

impl From<&FileHeader> for FileInfo {
    fn from(file_header: &FileHeader) -> Self {
        Self {
            compression: file_header.get_compression(),
            encryption: Encryption::None,
            filename: String::new(),
            data_checksum: file_header.data_checksum.clone(),
            raw_checksum: file_header.raw_checksum.clone()
        }
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