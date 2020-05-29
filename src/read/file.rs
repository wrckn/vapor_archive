use crate::{
    shared::{
        compression::Compression,
        error::{
            Error,
            Result
        },
        file_metadata::FileMetadata
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
    fs::Metadata,
    ops::{
        Range,
        
    },
};

/// Represents a single File in the archive
pub struct File<'r, R: Read + Seek + 'r> {
    /// An internal reference to the archives' source.
    comp_reader: CompReader<'r, R>,
    /// A counter for how many bytes were written.
    raw_bytes_read: usize,
    /// Total size of the uncompressed data.
    raw_size: usize,
    /// Optional Metadata override
    metadata_opt: Option<FileMetadata>
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
                raw_bytes_read: 0,
                raw_size: raw_size as usize,
                metadata_opt: None
            }
        )
    }

    /// ...with a given raw metadata
    pub fn with_metadata(mut self, metadata_ref: &Metadata) -> Self {
        self.metadata_opt = Some(metadata_ref.into());
        self
    }
}

impl<'r, R: Read + Seek + 'r> Read for File<'r, R> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let buffer_length = buf.len();
        let remaining_data = self.raw_size - self.raw_bytes_read;
        let read_length = if buffer_length > remaining_data {
            remaining_data
        } else {
            buffer_length
        };
        if read_length <= 0 {
            Ok(0)
        } else {
            self.comp_reader.read_exact(&mut buf[0..read_length])?;
            self.raw_bytes_read += read_length;
            Ok(read_length)
        }
    }
}