use spinel_macros::packet;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarInt;
use std::io::{self, Read, Write};

#[packet(id: "tag_query", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct TagQueryPacket {
    pub transaction_id: VarInt,
    pub data: QueryTagData,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct QueryTagData(pub Option<NbtCompound>);

impl DataType for QueryTagData {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match &self.0 {
            Some(data) => data.encode(writer),
            None => 0u8.encode(writer),
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let tag_id = u8::decode(reader)?;
        if tag_id == 0 {
            return Ok(Self(None));
        }
        let tag_prefix = [tag_id];
        let mut prefixed_reader = tag_prefix.as_slice().chain(reader);
        match Nbt::read_unnamed(&mut prefixed_reader)? {
            Nbt::Compound(compound) => Ok(Self(Some(compound))),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected query NBT compound",
            )),
        }
    }
}

impl TagQueryPacket {
    pub fn new(transaction_id: i32, data: Option<NbtCompound>) -> Self {
        Self {
            transaction_id,
            data: QueryTagData(data),
        }
    }
}
