use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageSignature {
    pub bytes: [u8; 256],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedMessageSignature {
    pub signature_cache_id: i32,
    pub full_signature: Option<MessageSignature>,
}

impl DataType for MessageSignature {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.bytes)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut bytes = [0; 256];
        reader.read_exact(&mut bytes)?;
        Ok(Self { bytes })
    }
}

impl DataType for PackedMessageSignature {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.signature_cache_id + 1).encode(writer)?;
        if let Some(full_signature) = &self.full_signature {
            full_signature.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let signature_cache_id = VarIntWrapper::decode(reader)?.0 - 1;
        let full_signature = if signature_cache_id == -1 {
            Some(MessageSignature::decode(reader)?)
        } else {
            None
        };

        Ok(Self {
            signature_cache_id,
            full_signature,
        })
    }
}
