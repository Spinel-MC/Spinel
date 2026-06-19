use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestBlockMode {
    Start,
    Log,
    Fail,
    Accept,
}

impl TestBlockMode {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Start => 0,
            Self::Log => 1,
            Self::Fail => 2,
            Self::Accept => 3,
        }
    }

    pub fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::Start),
            1 => Ok(Self::Log),
            2 => Ok(Self::Fail),
            3 => Ok(Self::Accept),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown test block mode protocol id {protocol_id}"),
            )),
        }
    }
}

impl DataType for TestBlockMode {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(VarIntWrapper::decode(reader)?.0)
    }
}
