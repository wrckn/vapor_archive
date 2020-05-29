#![warn(missing_docs)]

//! # `vapor_archive` - a .ZIP clone written in Rust

//! ## About

//! `vapor_archive` is, for the most part, a clone of the ZIP file format, with a few differences:
//! * File headers are stored after the data (making them footers),  
//! but the file format doesnt care about this
//! * Uses BLAKE2S instead of CRC32 for the checksums
//! * Supports ZStandard compression

//! ## Why?

//! No real reason really. I just needed my own archive format for my game engine that i could design as i please.

//! ## How does it work?

//! Once i get around to it, you will find the file format structure/specification [here.](SPEC.md)

//! ## How do i use it?

//! See the integration tests at `tests/` for a pointer. I will update the documentation as soon as im able too.


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

/// Full version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
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

/// Gets the library version
/// 
/// As a 16B long UTF-8 encoded string, formatted
/// as a semantic version number
pub fn get_version_bytes() -> [u8; 16] {
    let mut ret = [b" "[0]; 16];
    let version_bytes = VERSION.as_bytes();
    if version_bytes.len() > ret.len() {
        panic!("ERROR! Library version would not fit into header!");
    }
    for i in 0..version_bytes.len() {
        ret[i] = version_bytes[i];
    }
    ret
}