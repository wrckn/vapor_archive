use serde::{
    Serialize,
    Deserialize
};

/// Encryption type enum
/// 
/// Represents the different available Encryption types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Encryption {
    /// No encryption
    None,
    /// AES
    AES,
    /// Blowfish
    Blowfish,
    /// Twofish
    Twofish,
    /// Unknown/Unsupported
    Unknown,
}

impl From<u8> for Encryption {
    fn from(data: u8) -> Self {
        match data {
            0 => Encryption::None,
            1 => Encryption::AES,
            2 => Encryption::Blowfish,
            3 => Encryption::Twofish,
            _ => Encryption::Unknown
        }
    }
}

impl Default for Encryption {
    fn default() -> Self {
        Encryption::None
    }
}