use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitSet(pub Vec<u8>);

impl DataType for BitSet {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let num_longs = (self.0.len() + 7) / 8;
        VarIntWrapper(num_longs as i32).encode(w)?;

        let mut padded = self.0.clone();
        padded.resize(num_longs * 8, 0);
        w.write_all(&padded)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let num_longs = VarIntWrapper::decode(r)?.0 as usize;
        let mut buf = vec![0u8; num_longs * 8];
        r.read_exact(&mut buf)?;
        Ok(BitSet(buf))
    }
}
