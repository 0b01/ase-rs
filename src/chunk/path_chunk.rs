use std::io::{self, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct PathChunk {}

impl PathChunk {
    pub fn from_read<R>(read: &mut R, chunk_data_size: u32) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        read.seek(SeekFrom::Current(i64::from(chunk_data_size)))?;
        Ok(Self {})
    }

    pub fn write<W>(&self, _wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        Ok(())
    }
}
