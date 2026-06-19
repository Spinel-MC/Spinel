use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandSuggestionsRequestPacket {
    pub transaction_id: i32,
    pub text: String,
}

impl CommandSuggestionsRequestPacket {
    pub const fn get_id_const() -> i32 {
        0x0e
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for CommandSuggestionsRequestPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.transaction_id).encode(writer)?;
        self.text.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            transaction_id: VarIntWrapper::decode(reader)?.0,
            text: String::decode(reader)?,
        })
    }
}

impl PacketStruct for CommandSuggestionsRequestPacket {
    fn get_id() -> i32 {
        0x0e
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(
    CommandSuggestionsRequestPacket,
    spinel_network::Recipient::Server
);
