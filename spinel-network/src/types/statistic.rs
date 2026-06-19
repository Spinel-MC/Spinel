use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statistic {
    pub stat_type_id: i32,
    pub stat_value_id: i32,
}

impl DataType for Statistic {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.stat_type_id).encode(writer)?;
        VarIntWrapper(self.stat_value_id).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            stat_type_id: VarIntWrapper::decode(reader)?.0,
            stat_value_id: VarIntWrapper::decode(reader)?.0,
        })
    }
}
