use crate::data_type::DataType;
use spinel_registry::Identifier;
use std::io::{self, Read, Write};

impl DataType for Identifier {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.to_string().encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let identifier = String::decode(reader)?;
        identifier
            .parse()
            .map_err(|error: String| io::Error::new(io::ErrorKind::InvalidData, error))
    }
}
