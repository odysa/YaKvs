use crate::{error::Result, log_writer::OffSet, Command};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

pub struct PosReader<T: Seek> {
    reader: BufReader<T>,
    pos: u64,
}

impl<T: Seek> Seek for PosReader<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

impl<T: Seek> PosReader<T> {
    fn new(mut reader: BufReader<T>) -> Result<Self> {
        let pos = reader.seek(SeekFrom::Start(0))?;
        Ok(PosReader { pos, reader })
    }

    fn deserialize(&self, v: &[u8]) -> Result<Command> {
        Ok(serde_json::from_slice(v)?)
    }
}

impl PosReader<File> {
    fn read_command(&mut self, offset: OffSet) -> Result<Command> {
        self.reader.seek(SeekFrom::Start(offset.start()))?;
        let mut buffer = vec![0u8; offset.len() as usize];
        self.reader.read_exact(&mut buffer)?;

        match self.deserialize(&buffer) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}
