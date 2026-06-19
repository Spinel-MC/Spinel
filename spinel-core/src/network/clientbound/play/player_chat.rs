use spinel_network::data_type::DataType;
use spinel_network::types::{
    ChatType, FilterMask, MessageSignature, SignedMessageBodyPacked, var_int::VarIntWrapper,
};
use spinel_network::wrappers::NbtTextComponent;
use spinel_network::{ConnectionState, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerChatPacket {
    pub global_index: i32,
    pub sender: Uuid,
    pub index: i32,
    pub signature: Option<MessageSignature>,
    pub body: SignedMessageBodyPacked,
    pub unsigned_content: Option<TextComponent>,
    pub filter_mask: FilterMask,
    pub chat_type: ChatType,
}

impl DataType for PlayerChatPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.global_index).encode(writer)?;
        self.sender.encode(writer)?;
        VarIntWrapper(self.index).encode(writer)?;
        self.signature.encode(writer)?;
        self.body.encode(writer)?;
        self.unsigned_content
            .clone()
            .map(NbtTextComponent)
            .encode(writer)?;
        self.filter_mask.encode(writer)?;
        self.chat_type.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            global_index: VarIntWrapper::decode(reader)?.0,
            sender: Uuid::decode(reader)?,
            index: VarIntWrapper::decode(reader)?.0,
            signature: Option::<MessageSignature>::decode(reader)?,
            body: SignedMessageBodyPacked::decode(reader)?,
            unsigned_content: Option::<NbtTextComponent>::decode(reader)?.map(|content| content.0),
            filter_mask: FilterMask::decode(reader)?,
            chat_type: ChatType::decode(reader)?,
        })
    }
}

impl PacketStruct for PlayerChatPacket {
    fn get_id() -> i32 {
        0x3F
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(PlayerChatPacket, spinel_network::Recipient::Client);
