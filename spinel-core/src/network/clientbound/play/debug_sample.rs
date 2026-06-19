use spinel_network::data_type::DataType;
use spinel_network::{ConnectionState, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemoteDebugSampleType {
    TickTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugSamplePacket {
    pub sample: Vec<i64>,
    pub debug_sample_type: RemoteDebugSampleType,
}

impl RemoteDebugSampleType {
    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::TickTime),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown remote debug sample type {protocol_id}"),
            )),
        }
    }

    const fn protocol_id(self) -> i32 {
        match self {
            Self::TickTime => 0,
        }
    }
}

impl DataType for RemoteDebugSampleType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(VarIntWrapper::decode(reader)?.0)
    }
}

impl DataType for DebugSamplePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.sample.encode(writer)?;
        self.debug_sample_type.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            sample: Vec::<i64>::decode(reader)?,
            debug_sample_type: RemoteDebugSampleType::decode(reader)?,
        })
    }
}

impl PacketStruct for DebugSamplePacket {
    fn get_id() -> i32 {
        0x1E
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(DebugSamplePacket, spinel_network::Recipient::Client);
