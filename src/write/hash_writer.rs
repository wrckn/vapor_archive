use std::{
    io::{
        Write,
        Seek,
        SeekFrom,
        Result as IoResult
    },
    marker::PhantomData
};

use blake2::{
    Blake2s,
    Digest
};

pub struct HashWriter<'w, W: Write + Seek + 'w> {
    writer: &'w mut W,
    hasher: Blake2s,
    phantom_data: PhantomData<&'w W>
}

impl<'w, W: Write + Seek + 'w> HashWriter<'w, W> {
    pub fn new(writer: &'w mut W) -> Self {
        Self {
            writer: writer,
            hasher: Blake2s::new(),
            phantom_data: PhantomData::default()
        }
    }

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