use std::{
    io::{
        Seek,
        Read,
        Write,
        SeekFrom
    },
    marker::PhantomData
};

use crate::{
    shared::{
        compression::Compression,
        encryption::Encryption,
        directory::Directory,
        archive_header::ArchiveHeader,
        file_header::FileHeader,
        error::{
            Error,
            Result
        }
    },
    write::{
        file::File
    }
};
/// The Writer struct
/// 
/// Used for creating .var Archives
pub struct Writer<W: Write + Seek> {
    sink: W,
    directory: Directory
}

/// Creates a new Writer, wrapping a given Write struct
impl<'w, W: Write + Seek> Writer<W> {
    pub fn new(mut sink: W) -> Result<Writer<W>> {
        sink.write(b"VAR")
            .map_err(|_| Error::CantWriteMagicBytes)?;
        let archive_header = ArchiveHeader::default();
        bincode::serialize_into(&mut sink, &archive_header)
            .map_err(|_| Error::CantWriteHeader)?;
        Ok(
            Self {
                sink: sink,
                directory: Directory::default()
            }
        )
    }
    
    /// Initiates a new file writer at the given path
    pub fn write_file(&'w mut self, path: &str, compression_type: Compression) -> Result<File<'w, W>> {
        File::new(&mut self.sink, &mut self.directory,path, compression_type, Encryption::default())
    }
    
    /// Finishes and drops the archive writer
    pub fn finish(self) {}
}

impl<'w, W: Write + Seek> Drop for Writer<W> {
    fn drop(&mut self) {
        let directory_begin = self.sink.seek(SeekFrom::End(0)).unwrap();
        bincode::serialize_into(&mut self.sink, &self.directory).unwrap();
        let directory_end = self.sink.seek(SeekFrom::Current(0)).unwrap();
        self.sink.seek(SeekFrom::Start(3)).unwrap();
        let mut archive_header = ArchiveHeader::default();
        archive_header.directory = directory_begin..directory_end;
        bincode::serialize_into(&mut self.sink, &archive_header).unwrap();
    }
}