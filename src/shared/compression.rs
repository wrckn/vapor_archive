use serde::{
    Serialize,
    Deserialize
};

/// Compression enum. Describes the compression algorithm used.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Compression {
    /// No compression is used
    None,
    /// zstd compression
    ZStandard,
    /// deflate compression
    Deflate,
    /// LZMA compression
    LZMA,
    /// LZMA2 Compression
    LZMA2,
    /// BZIP2 Compression
    BZIP2,
    /// LZ4 Compression
    LZ4,
    /// Unknown compression
    Unknown
}

impl From<u8> for Compression {
    fn from(item: u8) -> Self {
        match item {
            0 => Compression::None,
            1 => Compression::ZStandard,
            _ => Compression::Unknown
        }
    }
}

impl Into<u8> for Compression {
    fn into(self) -> u8 {
        match self {
            Compression::None => 0,
            Compression::ZStandard => 1,
            _ => 255
        }
    }
}

impl Default for Compression {
    fn default() -> Self {
        Compression::ZStandard
    }
}