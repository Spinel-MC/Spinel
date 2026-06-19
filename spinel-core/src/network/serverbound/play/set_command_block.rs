use spinel_network::DataType;
use spinel_network::types::Position;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCommandBlockPacket {
    pub position: Position,
    pub command: String,
    pub mode: CommandBlockMode,
    pub flags: CommandBlockFlags,
}

impl SetCommandBlockPacket {
    pub const fn get_id_const() -> i32 {
        0x35
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for SetCommandBlockPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.position.encode(writer)?;
        self.command.encode(writer)?;
        self.mode.encode(writer)?;
        self.flags.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            position: Position::decode(reader)?,
            command: String::decode(reader)?,
            mode: CommandBlockMode::decode(reader)?,
            flags: CommandBlockFlags::decode(reader)?,
        })
    }
}

impl PacketStruct for SetCommandBlockPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(SetCommandBlockPacket, spinel_network::Recipient::Server);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandBlockMode {
    Sequence,
    Auto,
    Redstone,
}

impl DataType for CommandBlockMode {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(match self {
            Self::Sequence => 0,
            Self::Auto => 1,
            Self::Redstone => 2,
        })
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Sequence),
            1 => Ok(Self::Auto),
            2 => Ok(Self::Redstone),
            protocol_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown command block mode protocol id {protocol_id}"),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandBlockFlags {
    pub track_output: bool,
    pub conditional: bool,
    pub automatic: bool,
}

impl DataType for CommandBlockFlags {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let track_output_flag = if self.track_output { 1 } else { 0 };
        let conditional_flag = if self.conditional { 2 } else { 0 };
        let automatic_flag = if self.automatic { 4 } else { 0 };
        (track_output_flag | conditional_flag | automatic_flag).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let flags = u8::decode(reader)?;
        Ok(Self {
            track_output: flags & 1 != 0,
            conditional: flags & 2 != 0,
            automatic: flags & 4 != 0,
        })
    }
}
