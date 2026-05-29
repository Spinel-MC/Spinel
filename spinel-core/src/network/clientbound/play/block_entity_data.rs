use spinel_macros::packet;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::data_type::DataType;
use spinel_network::types::Position;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_registry::block_entity_type::BlockEntityType;
use std::io::{self, Read, Write};

#[packet(id: "block_entity_data", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct BlockEntityDataPacket {
    pub block_position: Position,
    pub block_entity_type: BlockEntityTypeNetworkId,
    pub data: BlockEntityNbt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockEntityTypeNetworkId(pub BlockEntityType);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlockEntityNbt(pub Option<NbtCompound>);

impl DataType for BlockEntityTypeNetworkId {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.0.id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let block_entity_type_id = VarIntWrapper::decode(reader)?.0;
        let block_entity_type =
            BlockEntityType::from_id(block_entity_type_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown block entity type: {block_entity_type_id}"),
                )
            })?;
        Ok(Self(block_entity_type))
    }
}

impl DataType for BlockEntityNbt {
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
                "Expected block entity NBT compound",
            )),
        }
    }
}

impl BlockEntityDataPacket {
    pub fn new(
        block_position: Position,
        block_entity_type: BlockEntityType,
        data: Option<NbtCompound>,
    ) -> Self {
        Self {
            block_position,
            block_entity_type: BlockEntityTypeNetworkId(block_entity_type),
            data: BlockEntityNbt(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BlockEntityDataPacket;
    use spinel_network::DataType;
    use spinel_network::types::Position;
    use spinel_registry::block_entity_type::BlockEntityType;

    #[test]
    fn block_entity_data_packet_matches_minestom_position_type_and_nbt_shape() {
        let packet =
            BlockEntityDataPacket::new(Position { x: 1, y: 2, z: 3 }, BlockEntityType::Chest, None);
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = BlockEntityDataPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(BlockEntityDataPacket::get_id(), 0x06);
        assert_eq!(decoded_packet.block_position, packet.block_position);
        assert_eq!(decoded_packet.block_entity_type, packet.block_entity_type);
        assert_eq!(decoded_packet.data, packet.data);
        assert_eq!(payload.last().copied(), Some(0));
    }
}
