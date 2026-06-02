use crate::network::resource_pack::ResourcePackStatus;
use spinel_network::{ConnectionState, DataType, PacketStruct};
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourcePackStatusPacket {
    pub id: Uuid,
    pub status: ResourcePackStatus,
}

impl ResourcePackStatusPacket {
    pub const fn get_id() -> i32 {
        0x06
    }

    pub const fn get_id_const() -> i32 {
        0x06
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Configuration
    }
}

impl DataType for ResourcePackStatusPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id.encode(writer)?;
        self.status.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            id: Uuid::decode(reader)?,
            status: ResourcePackStatus::decode(reader)?,
        })
    }
}

impl PacketStruct for ResourcePackStatusPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Configuration
    }
}
