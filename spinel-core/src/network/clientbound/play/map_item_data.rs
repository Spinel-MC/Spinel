use spinel_network::data_type::DataType;
use spinel_network::types::{MapColorPatch, MapDecoration, MapId};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct MapItemDataPacket {
    pub map_id: MapId,
    pub scale: i8,
    pub locked: bool,
    pub decorations: Option<Vec<MapDecoration>>,
    pub color_patch: Option<MapColorPatch>,
}

impl DataType for MapItemDataPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.map_id.encode(writer)?;
        self.scale.encode(writer)?;
        self.locked.encode(writer)?;
        self.decorations.encode(writer)?;
        MapColorPatch::encode_optional(&self.color_patch, writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            map_id: MapId::decode(reader)?,
            scale: i8::decode(reader)?,
            locked: bool::decode(reader)?,
            decorations: Option::<Vec<MapDecoration>>::decode(reader)?,
            color_patch: MapColorPatch::decode_optional(reader)?,
        })
    }
}

impl PacketStruct for MapItemDataPacket {
    fn get_id() -> i32 {
        0x31
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(MapItemDataPacket, spinel_network::Recipient::Client);
