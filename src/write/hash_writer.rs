use std::{
    io::{
        Write,
        Seek,
        SeekFrom,
        Result as IoResult
    }
};

use blake2::{
    Blake2s,
    Digest
};

/// HashWriter struct
/// 
/// Wraps another writer and recors the written bytes to
/// a BLAKE2S hasher, whose hash you can retrieve later
pub struct HashWriter<'w, W: Write + Seek + 'w> {
    /// A reference to the underlying writer
    writer: &'w mut W,
    /// The BLAKE2S hasher
    hasher: Blake2s
}

impl<'w, W: Write + Seek + 'w> HashWriter<'w, W> {
    /// Creates a new instance, wrapping the given writer
    pub fn new(writer: &'w mut W) -> Self {
        Self {
            writer: writer,
            hasher: Blake2s::new()
        }
    }
    
    /// Retrieves the BLAKE2S hash as a 32B array, resetting the hasher
    pub fn get_hash(&mut self) -> [u8; 32] {
        self.hasher.result_reset().into()
    }
}

impl<'w, W: Write + Seek + 'w> Write for HashWriter<'w, W> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let written  = self.writer.write(buf)?;
        self.hasher.input(&buf[0..written]);
        Ok(written)
    }
    fn flush(&mut self) -> IoResult<()> {
        self.writer.flush()
    }
}

impl<'w, W: Write + Seek + 'w> Seek for HashWriter<'w, W> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        self.writer.seek(pos)
    }
}