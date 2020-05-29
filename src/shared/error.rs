use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    },
    result::Result as StdResult,
    error::Error as ErrorTrait
};

/// Library-wide error type
#[derive(Debug)]
pub enum Error {
    /// And unknown error has occured
    Unknown,
    /// Cant read the magic bytes
    CantReadMagicBytes,
    /// Cant write the magic bytes
    CantWriteMagicBytes,
    /// Incorrect first 3 bytes
    IncorrectMagicBytes([u8; 3]),
    /// Incorrect version
    VersionMismatch((u8, u8, u8), (u8, u8, u8)),
    /// Cant read header
    CantReadHeader,
    /// Cant write header
    CantWriteHeader,
    /// Corrupt archive header
    CorruptHeader,
    /// Corrupt file header
    CorruptFileHeader,
    /// Corrupt directory
    CorruptDirectory,
    /// Couldnt read directory
    CantReadDirectory,
    /// Couldnt write directory
    CantWriteDirectory,
    /// Couldnt read file
    CantReadFile,
    /// Couldnt write file
    CantWriteFile,
    /// Couldnt read file header
    CantReadFileHeader,
    /// Couldnt write file header
    CantWriteFileHeader,
    /// Couldnt find the file specified
    FileNotFound(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl ErrorTrait for Error { }

/// Library-wide result type
pub type Result<T> = StdResult<T, Error>;