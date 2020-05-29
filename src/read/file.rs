use crate::{
    shared::{
        compression::Compression,
        error::{
            Error,
            Result
        }
    },
    read::{
        comp_reader::CompReader
    }
};

use std::{
    io::{
        Read,
        Seek,
        Result as IOResult,
        SeekFrom
    },
    ops::{
        Range,
        
    },
};

/// Represents a single File in the archive
pub struct File<'r, R: Read + Seek + 'r> {
    /// An internal reference to the archives' source.
    comp_reader: CompReader<'r, R>,
    /// A counter for how many bytes were written.
    data_read: usize,
    /// Total size of the data.
    data_size: usize,
    /// Total size of the uncompressed data.
    raw_size: usize
}

impl<'r, R: Read + Seek + 'r> File<'r, R> {
    /// Creates a new File
    /// 
    /// Given a reference to the underyling reader, a data range and the compression type
    pub fn new(reader: &'r mut R, raw_size: u64, data_range: Range<u64>, compression_type: Compression) -> Result<File<'r, R>> {
        reader.seek(SeekFrom::Start(data_range.start))
            .map_err(|_| Error::CantReadFile)?;
        let comp_reader: CompReader<'r, R> = CompReader::new(reader, compression_type);
        Ok(
            Self {
                comp_reader: comp_reader,
                data_read: 0,
                data_size: (data_range.end - data_range.start) as usize,
                raw_size: raw_size as usize
            }
        )
    }
}

impl<'r, R: Read + Seek + 'r> Read for File<'r, R> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let buffer_length = buf.len();
        let remaining_data = self.raw_size - self.data_read;
        let read_length = if buffer_length > remaining_data {
            remaining_data
        } else {
            buffer_length
        };
        if read_length <= 0 {
            Ok(0)
        } else {
            self.comp_reader.read_exact(&mut buf[0..read_length])?;
            self.data_read += read_length;
            Ok(read_length)
        }
    }
}