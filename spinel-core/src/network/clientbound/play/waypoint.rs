use spinel_network::data_type::DataType;
use spinel_network::types::{TrackedWaypoint, var_int::VarIntWrapper};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaypointOperation {
    Track,
    Untrack,
    Update,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaypointPacket {
    pub operation: WaypointOperation,
    pub waypoint: TrackedWaypoint,
}

impl WaypointOperation {
    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::Track),
            1 => Ok(Self::Untrack),
            2 => Ok(Self::Update),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown waypoint operation {protocol_id}"),
            )),
        }
    }

    const fn protocol_id(self) -> i32 {
        match self {
            Self::Track => 0,
            Self::Untrack => 1,
            Self::Update => 2,
        }
    }
}

impl DataType for WaypointOperation {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(VarIntWrapper::decode(reader)?.0)
    }
}

impl DataType for WaypointPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.operation.encode(writer)?;
        self.waypoint.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            operation: WaypointOperation::decode(reader)?,
            waypoint: TrackedWaypoint::decode(reader)?,
        })
    }
}

impl PacketStruct for WaypointPacket {
    fn get_id() -> i32 {
        0x88
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(WaypointPacket, spinel_network::Recipient::Client);
