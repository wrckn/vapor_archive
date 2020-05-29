use std::{
    io::{
        Write,
        Seek
    }
};

use zstd::{
    stream::AutoFinishEncoder
};

/// Supertrait of Write and Seek
pub trait Sink: Write + Seek {}

impl<W: Write + Seek> Sink for W {}