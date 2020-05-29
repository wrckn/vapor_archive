use crate::{
    shared::{
        error::{
            Error,
            Result
        },
        file_header::FileHeader,
        file_info::FileInfo,
        compression::Compression,
        encryption::Encryption,
        directory::Directory
    },
    write::{
        comp_writer::CompWriter
    }
};

use std::{
    io::{
        Write,
        Seek,
        SeekFrom,
        Result as IoResult,
    },
    ops::{
        Drop
    }
};

use blake2::{
    Blake2s,
    Digest
};

/// File
/// 
/// Represents a File stream within the archive.
/// This wraps the internal writer within several layers of
/// supported encryption/decryption.
pub struct File<'w, W: Write + Seek + 'w> {
    /// The compression writer. Corresponds to a supported compression type.
    comp_writer: CompWriter<'w, W>,
    /// Byte range of the data.
    data_begin: u64,
    /// Hasher for creating the raw checksum
    raw_hasher: Blake2s,
    /// Raw data size
    raw_size: usize,
    /// Filename,
    filename: String,
    /// Directory pointer
    directory: &'w mut Directory
}

impl<'w, W: Write + Seek + 'w> File<'w, W> {
    /// Creates a new File Writer
    /// 
    /// Wraps a writer and creates a new file with the given parameters
    pub fn new(writer: &'w mut W, directory: &'w mut Directory, filename: &str, compression_type: Compression, _encryption_type: Encryption) -> Result<Self> {
        let data_start = writer.seek(SeekFrom::Current(0)).map_err(|_| Error::Unknown)?;
        println!("BYTE OFFSET OF THIS FILE: {}", data_start);
        Ok(
            Self {
                comp_writer: CompWriter::new(writer, compression_type),
                data_begin: data_start,
                filename: String::from(filename),
                raw_hasher: Blake2s::new(),
                raw_size: 0,
                directory: directory
            }
        )
    }

    /// Drops the File Writer early, writing the header
    pub fn finish(self) {}
}

impl<'w, W: Write + Seek + 'w> Write for File<'w, W> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let written  = self.comp_writer.write(buf)?;
        self.raw_size += written;
        self.raw_hasher.input(&buf[0..written]);
        Ok(written)
    }
    
    fn flush(&mut self) -> IoResult<()> {
        self.comp_writer.flush()
    }
}

impl<'w, W: Write + Seek + 'w> Drop for File<'w, W> {
    fn drop(&mut self) {
        let compression_type = self.comp_writer.get_compression_type();
        let raw_checksum: [u8; 32] = self.raw_hasher.result_reset().into();
        let data_checksum = self.comp_writer.get_data_hash();
        let data_end = self.comp_writer.seek(SeekFrom::Current(0)).unwrap();
        println!("DATA SIZE OF THIS FILE: {}", data_end - self.data_begin);
        let file_header_begin = data_end;
        let file_info = FileInfo::default()
            .with_compression(compression_type)
            .with_raw_size(self.raw_size as u64)
            .with_data_size(data_end - self.data_begin)
            .with_raw_checksum(raw_checksum)
            .with_data_checksum(data_checksum);
        let file_header = FileHeader::default()
            .with_data_range(self.data_begin..data_end)
            .with_file_info(file_info);
        bincode::serialize_into(&mut self.comp_writer, &file_header).unwrap();
        let file_header_end = self.comp_writer.seek(SeekFrom::Current(0)).unwrap();
        println!("Position after serialize: {}", file_header_end);
        println!("Size difference: {}", file_header_end - file_header_begin);
        println!("Written file header from byte #{} to #{}.", file_header_begin, file_header_end);
        println!("RAW SIZE OF THIS FILE: {}", self.raw_size);
        self.directory.set_file(&self.filename, file_header_begin..file_header_end);
    }
}