#![warn(missing_docs)]
//!
//! VaporArchive - An Archive format for vapor_engine
//!

pub extern crate blake2;

/// Read functionality module
pub mod read;

/// Write functionality module
pub mod write;

/// Shared functionality module
pub mod shared;

pub use read::archive::Archive as VarArchive;
pub use read::file::File as VarFile;
pub use write::writer::Writer as VarWriter;
pub use shared::compression::Compression;

/// Major version
pub const VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
/// Minor version
pub const VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
/// Patch version
pub const VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

/// Gets the library version
/// 
/// In (MAJOR, MINOR, PATCH) format
pub fn get_version_tuple() -> (u8, u8, u8) {
    (
        VERSION_MAJOR.parse().expect("Incorrect version format!"),
        VERSION_MINOR.parse().expect("Incorrect version format!"),
        VERSION_PATCH.parse().expect("Incorrect version format!")
    )
}