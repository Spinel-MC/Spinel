use super::debug_entity_value::DebugSubscriptionUpdate;
use spinel_network::data_type::DataType;
use spinel_network::types::Position;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugBlockValuePacket {
    pub block_pos: Position,
    pub update: DebugSubscriptionUpdate,
}

impl DataType for DebugBlockValuePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.block_pos.encode(writer)?;
        self.update.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            block_pos: Position::decode(reader)?,
            update: DebugSubscriptionUpdate::decode(reader)?,
        })
    }
}

impl PacketStruct for DebugBlockValuePacket {
    fn get_id() -> i32 {
        0x1A
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(DebugBlockValuePacket, spinel_network::Recipient::Client);
