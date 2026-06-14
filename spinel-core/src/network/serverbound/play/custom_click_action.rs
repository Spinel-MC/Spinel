use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayCustomClickActionPacket {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

impl PlayCustomClickActionPacket {
    pub const fn get_id_const() -> i32 {
        0x41
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn payload_without_end_tag(&self) -> Option<&[u8]> {
        if self.payload.first() == Some(&0) {
            return None;
        }
        Some(&self.payload)
    }
}

impl DataType for PlayCustomClickActionPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.key.encode(writer)?;
        writer.write_all(&self.payload)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let key = Identifier::decode(reader)?;
        let mut payload = Vec::new();
        reader.read_to_end(&mut payload)?;
        Ok(Self { key, payload })
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
