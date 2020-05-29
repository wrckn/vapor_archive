use crate::{
    shared::{
        compression::Compression
    }
};

use std::{
    io::{
        Read,
        BufReader,
        Seek,
        Result as IoResult
    }
};

use zstd::{
    Decoder
};

/// CompReader Enum
/// 
/// Wraps a reader in various supported compression algorithms.
pub enum CompReader<'r, R: Read + Seek + 'r> {
    /// None - misconfigured reader.
    None,
    /// Raw. No compression.
    Raw(&'r mut R),
    /// Zstd. Comparable to DEFLATE, but much faster.
    ZStandard(Decoder<BufReader<&'r mut R>>),
}

impl<'r, R: Read + Seek + 'r> CompReader<'r, R> {
    /// Creates a new CompReader with a given type, wrapping a reader
    pub fn new(reader: &'r mut R, compression_type: Compression) -> Self {
        match compression_type {
            Compression::None => CompReader::Raw(reader),
            Compression::ZStandard => {
                let decoder = Decoder::new(reader).expect("Error creating Zstd decoder!");
                CompReader::ZStandard(decoder)
            },
            _ => unimplemented!("Unimplemented compression type!")
        }
    }
}

impl<'r, R: Read + Seek + 'r> Read for CompReader<'r, R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let reader_ref: &mut dyn Read = match self {
            CompReader::Raw(reader) => reader,
            CompReader::ZStandard(encoder) => encoder,
            _ => panic!("Misconfigured reader!")
        };
        reader_ref.read(buf)
    }
}