use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainHand {
    Left,
    Right,
}

impl DataType for MainHand {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let protocol_id = match self {
            Self::Left => 0,
            Self::Right => 1,
        };
        VarIntWrapper(protocol_id).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            protocol_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown main hand protocol id: {protocol_id}"),
            )),
        }
    }
}
