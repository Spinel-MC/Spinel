use crate::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockRotation {
    None,
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

impl DataType for BlockRotation {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(match self {
            Self::None => 0,
            Self::Clockwise90 => 1,
            Self::Clockwise180 => 2,
            Self::Counterclockwise90 => 3,
        })
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::None),
            1 => Ok(Self::Clockwise90),
            2 => Ok(Self::Clockwise180),
            3 => Ok(Self::Counterclockwise90),
            protocol_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown block rotation protocol id {protocol_id}"),
            )),
        }
    }
}
