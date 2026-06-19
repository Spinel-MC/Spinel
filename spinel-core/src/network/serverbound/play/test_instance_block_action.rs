use spinel_network::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::types::{BlockRotation, Position};
use spinel_network::wrappers::NbtTextComponent;
use spinel_network::{ConnectionState, PacketStruct};
use spinel_registry::Identifier;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct TestInstanceBlockActionPacket {
    pub position: Position,
    pub action: TestInstanceBlockAction,
    pub test_instance_data: TestInstanceBlockData,
}

impl TestInstanceBlockActionPacket {
    pub const fn get_id_const() -> i32 {
        0x3e
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for TestInstanceBlockActionPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.position.encode(writer)?;
        self.action.encode(writer)?;
        self.test_instance_data.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            position: Position::decode(reader)?,
            action: TestInstanceBlockAction::decode(reader)?,
            test_instance_data: TestInstanceBlockData::decode(reader)?,
        })
    }
}

impl PacketStruct for TestInstanceBlockActionPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(
    TestInstanceBlockActionPacket,
    spinel_network::Recipient::Server
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestInstanceBlockAction {
    Init,
    Query,
    Set,
    Reset,
    Save,
    Export,
    Run,
}

impl DataType for TestInstanceBlockAction {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(match self {
            Self::Init => 0,
            Self::Query => 1,
            Self::Set => 2,
            Self::Reset => 3,
            Self::Save => 4,
            Self::Export => 5,
            Self::Run => 6,
        })
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Init),
            1 => Ok(Self::Query),
            2 => Ok(Self::Set),
            3 => Ok(Self::Reset),
            4 => Ok(Self::Save),
            5 => Ok(Self::Export),
            6 => Ok(Self::Run),
            protocol_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown test instance block action protocol id {protocol_id}"),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestInstanceBlockData {
    pub test: Option<Identifier>,
    pub size: TestInstanceBlockSize,
    pub rotation: BlockRotation,
    pub ignore_entities: bool,
    pub status: TestInstanceBlockStatus,
    pub error_message: Option<TextComponent>,
}

impl DataType for TestInstanceBlockData {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.test.encode(writer)?;
        self.size.encode(writer)?;
        self.rotation.encode(writer)?;
        self.ignore_entities.encode(writer)?;
        self.status.encode(writer)?;
        self.error_message
            .clone()
            .map(NbtTextComponent)
            .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            test: Option::<Identifier>::decode(reader)?,
            size: TestInstanceBlockSize::decode(reader)?,
            rotation: BlockRotation::decode(reader)?,
            ignore_entities: bool::decode(reader)?,
            status: TestInstanceBlockStatus::decode(reader)?,
            error_message: Option::<NbtTextComponent>::decode(reader)?.map(|message| message.0),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestInstanceBlockSize {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl DataType for TestInstanceBlockSize {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.x).encode(writer)?;
        VarIntWrapper(self.y).encode(writer)?;
        VarIntWrapper(self.z).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            x: VarIntWrapper::decode(reader)?.0,
            y: VarIntWrapper::decode(reader)?.0,
            z: VarIntWrapper::decode(reader)?.0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestInstanceBlockStatus {
    Cleared,
    Running,
    Finished,
}

impl DataType for TestInstanceBlockStatus {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(match self {
            Self::Cleared => 0,
            Self::Running => 1,
            Self::Finished => 2,
        })
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Cleared),
            1 => Ok(Self::Running),
            2 => Ok(Self::Finished),
            protocol_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown test instance block status protocol id {protocol_id}"),
            )),
        }
    }
}
