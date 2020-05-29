use crate::{
    shared::{
        compression::Compression
    },
    write::{
        hash_writer::HashWriter
    }
};

use std::{
    io::{
        Write,
        Seek,
        SeekFrom,
        Result as IoResult
    }
};

use zstd::{
    Encoder
};

/// CompWriter enum
/// 
/// Wraps a writer in one of the supported compression algorithms.
pub enum CompWriter<'w, W: Write + Seek + 'w> {
    /// None - misconfigured Writer.
    None,
    /// Raw - no compression.
    Raw(HashWriter<'w, W>),
    /// Zstd compression.
    ZStandard(Encoder<HashWriter<'w, W>>)
}

impl<'w, W: Write + Seek + 'w> CompWriter<'w, W> {
    /// Creates a new CompWriter
    /// 
    /// Wraps a given writer first in a HashWriter and then in a CompWriter variant
    /// corresponding to the chosen compression algorithm.
    pub fn new(writer: &'w mut W, compression_type: Compression) -> Self {
        match compression_type {
            Compression::None => CompWriter::Raw(HashWriter::new(writer)),
            Compression::ZStandard => {
                let encoder = Encoder::new(HashWriter::new(writer), 9).expect("Error creating Zstd encoder!");
                CompWriter::ZStandard(encoder)
            },
            _ => unimplemented!("Not implemented!")
        }
    }
    
    /// Gets the compression type corresponding to this writer variant
    pub fn get_compression_type(&self) -> Compression {
        match self {
            CompWriter::Raw(_) => Compression::None,
            CompWriter::ZStandard(_) => Compression::ZStandard,
            _ => Compression::Unknown
        }
    }
    
    /// Gets the data hash, dropping the internal writer in the process
    /// 
    /// Only use this once you have finished writing.
    pub fn get_data_hash(&mut self) -> [u8; 32] {
        let mut hash_writer = match std::mem::replace(self, CompWriter::None) {
            CompWriter::Raw(hash_writer) => hash_writer,
            CompWriter::ZStandard(encoder) => encoder.finish().unwrap(),
            _ => panic!("ERROR")
        };
        let ret = hash_writer.get_hash();
        *self = CompWriter::Raw(hash_writer);
        ret
    }
}

impl<'w, W: Write + Seek + 'w> Write for CompWriter<'w, W> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let writer: &mut dyn Write = match self {
            CompWriter::Raw(hash_writer) => hash_writer,
            CompWriter::ZStandard(encoder) => encoder,
            _ => panic!("Writer misconfigured!")
        };
        writer.write(buf)
    }
    
    fn flush(&mut self) -> IoResult<()> {
        let writer: &mut dyn Write = match self {
            CompWriter::Raw(hash_writer) => hash_writer,
            CompWriter::ZStandard(encoder) => encoder,
            _ => panic!("Writer misconfigured!")
        };
        writer.flush()
    }
}

impl<'w, W: Write + Seek + 'w> Seek for CompWriter<'w, W> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        let writer: &mut dyn Seek = match self {
            CompWriter::Raw(hash_writer) => hash_writer,
            CompWriter::ZStandard(encoder) => encoder.get_mut(),
            _ => panic!("Writer misconfigured!")
        };
        writer.seek(pos)
    }
}