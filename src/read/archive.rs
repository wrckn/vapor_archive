use crate::{
    shared::{
        directory::Directory,
        archive_header::ArchiveHeader,
        file_header::FileHeader,
        file_info::FileInfo,
        error::{
            Error,
            Result
        }
    },
    read::{
        file::File
    }
};

use std::{
    io::{
        Read,
        Seek,
        SeekFrom
    },
    ops::Range
};


/// The Archive struct
pub struct Archive<R: Read + Seek> {
    /// The source - e.g. a file
    source: R,
    /// The directory
    directory: Directory,
}

impl<'r, R: Read + Seek + 'r> Archive<R> {
    /// Creates a new archive, wrapping the given source
    pub fn new(mut source: R) -> Result<Self> {
        // Read the magic bytes
        let mut magic_bytes = [0u8; 3];
        source.read_exact(&mut magic_bytes)
            .map_err(|_| Error::CantReadMagicBytes)?;
        if &magic_bytes != b"VAR" {
            return Err(Error::IncorrectMagicBytes(magic_bytes));
        }
        let header: ArchiveHeader = bincode::deserialize_from(&mut source)
            .map_err(|_| Error::CorruptHeader)?;
        // // Prepare the buffer
        // let mut directory_bytes: Vec<u8> = vec![];
        // let directory_len = (header.directory.end - header.directory.start) as usize;
        // directory_bytes.resize(directory_len, 0);
        // // Jump to the start of the directory
        // source.seek(SeekFrom::Start(header.directory.start))
        //     .map_err(|_| Error::Unknown)?;
        // // Read the bytes
        // source.read_exact(&mut directory_bytes)
        //     .map_err(|_| Error::CantReadDirectory)?;
        // // Parse the directory
        let dir_begin = header.directory_range.start;
        source.seek(SeekFrom::Start(dir_begin))
            .map_err(|_| Error::CantReadDirectory)?;
        let directory: Directory = bincode::deserialize_from(&mut source)
            .map_err(|_| Error::CorruptDirectory)?;
        Ok(
            Self {
                directory: directory,
                source: source
            }
        )
    }

    /// Lists all files
    pub fn get_file_list(&self) -> Vec<String> {
        self.directory.get_file_list()
    }
    
    /// Gets a File stream by path
    pub fn get_file(&'r mut self, path: &str) -> Result<File<'r, R>> {
        let file_header_range = self.directory.get_file(path)
            .ok_or(Error::FileNotFound(String::from(path)))?;
        let file_header = self.get_file_header(file_header_range)?;
        File::new(&mut self.source, file_header.file_info.raw_size, file_header.data_range.clone(), file_header.file_info.compression.clone())
    }

    /// Gets a files info by path
    pub fn get_file_info(&mut self, path: &str) -> Result<FileInfo> {
        let file_header_range = self.directory.get_file(path)
            .ok_or(Error::FileNotFound(String::from(path)))?;
        let file_header = self.get_file_header(file_header_range)?;
        let file_info = file_header.file_info.clone()
            .with_filename(path);
        Ok(
            file_info
        )
    }

    /// Gets a file header
    fn get_file_header(&mut self, file_header_range: Range<u64>) -> Result<FileHeader> {
        let file_header_start = file_header_range.start;
        let file_header_range = file_header_range.start as usize .. file_header_range.end as usize;
        let file_header_size = file_header_range.end - file_header_range.start;
        let mut file_header_bytes = vec![];
        file_header_bytes.resize(file_header_size, 0);
        self.source.seek(SeekFrom::Start(file_header_start))
            .map_err(|_| Error::CantReadFileHeader)?;
        self.source.read_exact(&mut file_header_bytes)
            .map_err(|_| Error::CantReadFileHeader)?;
        bincode::deserialize(&file_header_bytes)
            .map_err(|_| Error::CorruptFileHeader)
    }
}