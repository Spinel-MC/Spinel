use spinel_network::{ConnectionState, DataType, PacketSender, PacketStruct};
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourcePackPopPacket {
    pub id: Option<Uuid>,
}

impl ResourcePackPopPacket {
    pub const fn get_id() -> i32 {
        0x08
    }

    pub const fn new(id: Option<Uuid>) -> Self {
        Self { id }
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let mut payload = Vec::new();
        self.encode(&mut payload)?;
        sender.send_packet(Self::get_id(), &payload)
    }
}

impl DataType for ResourcePackPopPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            id: Option::<Uuid>::decode(reader)?,
        })
    }
}

impl PacketStruct for ResourcePackPopPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Configuration
    }
}
spinel_network::register_packet_codec!(ResourcePackPopPacket, spinel_network::Recipient::Client);
