use crate::data_type::DataType;
use std::io::{self, Write};

#[derive(Default)]
pub struct NetworkBuffer {
    pub buffer: Vec<u8>,
}

impl NetworkBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }

    pub fn encode<T: DataType>(&mut self, value: &T) -> io::Result<()> {
        value.encode(self)
    }
}

impl Write for NetworkBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
