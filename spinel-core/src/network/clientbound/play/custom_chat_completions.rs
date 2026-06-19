use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomChatCompletionsPacket {
    pub action: CustomChatCompletionsAction,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomChatCompletionsAction {
    Add,
    Remove,
    Set,
}

impl CustomChatCompletionsAction {
    const fn ordinal(self) -> i32 {
        match self {
            Self::Add => 0,
            Self::Remove => 1,
            Self::Set => 2,
        }
    }

    fn from_ordinal(ordinal: i32) -> io::Result<Self> {
        match ordinal {
            0 => Ok(Self::Add),
            1 => Ok(Self::Remove),
            2 => Ok(Self::Set),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown custom chat completions action ordinal {ordinal}"),
            )),
        }
    }
}

impl DataType for CustomChatCompletionsAction {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.ordinal()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_ordinal(VarIntWrapper::decode(reader)?.0)
    }
}

impl CustomChatCompletionsPacket {
    pub const fn get_id_const() -> i32 {
        0x17
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for CustomChatCompletionsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.action.encode(writer)?;
        self.entries.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            action: CustomChatCompletionsAction::decode(reader)?,
            entries: Vec::<String>::decode(reader)?,
        })
    }
}

impl PacketStruct for CustomChatCompletionsPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(
    CustomChatCompletionsPacket,
    spinel_network::Recipient::Client
);
