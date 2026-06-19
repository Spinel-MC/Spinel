use super::debug_entity_value::DebugSubscriptionUpdate;
use spinel_network::data_type::DataType;
use spinel_network::types::ChunkPos;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugChunkValuePacket {
    pub chunk_pos: ChunkPos,
    pub update: DebugSubscriptionUpdate,
}

impl DataType for DebugChunkValuePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.chunk_pos.encode(writer)?;
        self.update.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            chunk_pos: ChunkPos::decode(reader)?,
            update: DebugSubscriptionUpdate::decode(reader)?,
        })
    }
}

impl PacketStruct for DebugChunkValuePacket {
    fn get_id() -> i32 {
        0x1B
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(DebugChunkValuePacket, spinel_network::Recipient::Client);
