use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntList(pub Vec<i32>);

impl DataType for IntList {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.0.len() as i32).encode(w)?;
        for &value in &self.0 {
            VarIntWrapper(value).encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let len = VarIntWrapper::decode(r)?.0 as usize;
        if len > 1_000_000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "IntList length too large",
            ));
        }
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            list.push(VarIntWrapper::decode(r)?.0);
        }
        Ok(IntList(list))
    }
}
