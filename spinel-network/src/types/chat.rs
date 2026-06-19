use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatType {
    pub encoded_bound_chat_type_payload: Vec<u8>,
}

impl DataType for ChatType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.encoded_bound_chat_type_payload)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut encoded_bound_chat_type_payload = Vec::new();
        reader.read_to_end(&mut encoded_bound_chat_type_payload)?;
        Ok(Self {
            encoded_bound_chat_type_payload,
        })
    }
}
