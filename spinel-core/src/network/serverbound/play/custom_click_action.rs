use spinel_network::data_type::DataType;
use spinel_network::types::{CustomClickActionPayload, Identifier};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayCustomClickActionPacket {
    pub click_action_id: Identifier,
    pub payload: CustomClickActionPayload,
}

impl PlayCustomClickActionPacket {
    pub const fn get_id_const() -> i32 {
        0x41
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn payload_bytes(&self) -> Option<&[u8]> {
        self.payload.bytes()
    }
}

impl DataType for PlayCustomClickActionPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.click_action_id.encode(writer)?;
        self.payload.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            click_action_id: Identifier::decode(reader)?,
            payload: CustomClickActionPayload::decode(reader)?,
        })
    }
}

impl PacketStruct for PlayCustomClickActionPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}
spinel_network::register_packet_codec!(
    PlayCustomClickActionPacket,
    spinel_network::Recipient::Server
);
