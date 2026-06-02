use crate::network::resource_pack::ResourcePackInfo;
use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, DataType, PacketSender, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct ResourcePackPushPacket {
    pub id: Uuid,
    pub url: String,
    pub hash: String,
    pub forced: bool,
    pub prompt: Option<TextComponent>,
}

impl ResourcePackPushPacket {
    pub const fn get_id() -> i32 {
        0x09
    }

    pub fn new(
        resource_pack: &ResourcePackInfo,
        forced: bool,
        prompt: Option<TextComponent>,
    ) -> Self {
        Self {
            id: resource_pack.id(),
            url: resource_pack.url().to_string(),
            hash: resource_pack.hash().to_string(),
            forced,
            prompt,
        }
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let mut payload = Vec::new();
        self.encode(&mut payload)?;
        sender.send_packet(Self::get_id(), &payload)
    }
}

impl DataType for ResourcePackPushPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id.encode(writer)?;
        self.url.encode(writer)?;
        self.hash.encode(writer)?;
        self.forced.encode(writer)?;
        self.prompt.clone().map(JsonTextComponent).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            id: Uuid::decode(reader)?,
            url: String::decode(reader)?,
            hash: String::decode(reader)?,
            forced: bool::decode(reader)?,
            prompt: Option::<JsonTextComponent>::decode(reader)?.map(|prompt| prompt.0),
        })
    }
}

impl PacketStruct for ResourcePackPushPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Configuration
    }
}
