use spinel_network::data_type::DataType;
use spinel_network::types::ChatType;
use spinel_network::wrappers::NbtTextComponent;
use spinel_network::{ConnectionState, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct DisguisedChatPacket {
    pub message: TextComponent,
    pub chat_type: ChatType,
}

impl DataType for DisguisedChatPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        NbtTextComponent(self.message.clone()).encode(writer)?;
        self.chat_type.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            message: NbtTextComponent::decode(reader)?.0,
            chat_type: ChatType::decode(reader)?,
        })
    }
}

impl PacketStruct for DisguisedChatPacket {
    fn get_id() -> i32 {
        0x21
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(DisguisedChatPacket, spinel_network::Recipient::Client);
