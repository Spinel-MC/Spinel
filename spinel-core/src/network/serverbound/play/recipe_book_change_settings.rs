use spinel_network::types::RecipeBookType;
use spinel_network::{ConnectionState, DataType, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeBookChangeSettingsPacket {
    pub book_type: RecipeBookType,
    pub is_open: bool,
    pub is_filtering: bool,
}

impl RecipeBookChangeSettingsPacket {
    pub const fn get_id_const() -> i32 {
        0x2d
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for RecipeBookChangeSettingsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.book_type.encode(writer)?;
        self.is_open.encode(writer)?;
        self.is_filtering.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            book_type: RecipeBookType::decode(reader)?,
            is_open: bool::decode(reader)?,
            is_filtering: bool::decode(reader)?,
        })
    }
}

impl PacketStruct for RecipeBookChangeSettingsPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(
    RecipeBookChangeSettingsPacket,
    spinel_network::Recipient::Server
);
