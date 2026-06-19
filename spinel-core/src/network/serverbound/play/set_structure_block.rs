use spinel_network::DataType;
use spinel_network::types::var_long::VarLongWrapper;
use spinel_network::types::{
    BlockRotation, Position, StructureBlockMirror, StructureBlockMode, StructureBlockUpdateType,
};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct SetStructureBlockPacket {
    pub position: Position,
    pub update_type: StructureBlockUpdateType,
    pub mode: StructureBlockMode,
    pub name: String,
    pub offset: StructureBlockOffset,
    pub size: StructureBlockSize,
    pub mirror: StructureBlockMirror,
    pub rotation: BlockRotation,
    pub metadata: String,
    pub integrity: f32,
    pub seed: StructureBlockSeed,
    pub flags: StructureBlockFlags,
}

impl SetStructureBlockPacket {
    pub const fn get_id_const() -> i32 {
        0x39
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for SetStructureBlockPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.position.encode(writer)?;
        self.update_type.encode(writer)?;
        self.mode.encode(writer)?;
        self.name.encode(writer)?;
        self.offset.encode(writer)?;
        self.size.encode(writer)?;
        self.mirror.encode(writer)?;
        self.rotation.encode(writer)?;
        self.metadata.encode(writer)?;
        self.integrity.encode(writer)?;
        self.seed.encode(writer)?;
        self.flags.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            position: Position::decode(reader)?,
            update_type: StructureBlockUpdateType::decode(reader)?,
            mode: StructureBlockMode::decode(reader)?,
            name: String::decode(reader)?,
            offset: StructureBlockOffset::decode(reader)?,
            size: StructureBlockSize::decode(reader)?,
            mirror: StructureBlockMirror::decode(reader)?,
            rotation: BlockRotation::decode(reader)?,
            metadata: String::decode(reader)?,
            integrity: f32::decode(reader)?,
            seed: StructureBlockSeed::decode(reader)?,
            flags: StructureBlockFlags::decode(reader)?,
        })
    }
}

impl PacketStruct for SetStructureBlockPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(SetStructureBlockPacket, spinel_network::Recipient::Server);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructureBlockOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl DataType for StructureBlockOffset {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.x.encode(writer)?;
        self.y.encode(writer)?;
        self.z.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            x: i8::decode(reader)?,
            y: i8::decode(reader)?,
            z: i8::decode(reader)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructureBlockSize {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl DataType for StructureBlockSize {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.x.encode(writer)?;
        self.y.encode(writer)?;
        self.z.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            x: i8::decode(reader)?,
            y: i8::decode(reader)?,
            z: i8::decode(reader)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructureBlockSeed(pub i64);

impl DataType for StructureBlockSeed {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarLongWrapper(self.0).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        VarLongWrapper::decode(reader).map(|seed| Self(seed.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructureBlockFlags {
    pub ignore_entities: bool,
    pub strict: bool,
    pub show_air: bool,
    pub show_bounding_box: bool,
}

impl DataType for StructureBlockFlags {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let ignore_entities_flag = if self.ignore_entities { 1 } else { 0 };
        let show_air_flag = if self.show_air { 2 } else { 0 };
        let show_bounding_box_flag = if self.show_bounding_box { 4 } else { 0 };
        let strict_flag = if self.strict { 8 } else { 0 };
        (ignore_entities_flag | show_air_flag | show_bounding_box_flag | strict_flag).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let flags = u8::decode(reader)?;
        Ok(Self {
            ignore_entities: flags & 1 != 0,
            strict: flags & 8 != 0,
            show_air: flags & 2 != 0,
            show_bounding_box: flags & 4 != 0,
        })
    }
}
