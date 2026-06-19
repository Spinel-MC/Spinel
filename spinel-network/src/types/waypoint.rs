use crate::data_type::DataType;
use crate::types::{ChunkPos, Identifier, Vector3i, var_int::VarIntWrapper};
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackedWaypointId {
    EntityUuid(Uuid),
    Named(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaypointIcon {
    pub style_asset: Identifier,
    pub color: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrackedWaypoint {
    Empty {
        id: TrackedWaypointId,
        icon: WaypointIcon,
    },
    Position {
        id: TrackedWaypointId,
        icon: WaypointIcon,
        position: Vector3i,
    },
    Chunk {
        id: TrackedWaypointId,
        icon: WaypointIcon,
        chunk_pos: ChunkPos,
    },
    Azimuth {
        id: TrackedWaypointId,
        icon: WaypointIcon,
        angle_radians: f32,
    },
}

impl WaypointIcon {
    pub fn null() -> Self {
        Self {
            style_asset: Identifier::minecraft("default"),
            color: None,
        }
    }
}

impl TrackedWaypoint {
    const EMPTY_TYPE_ID: i32 = 0;
    const POSITION_TYPE_ID: i32 = 1;
    const CHUNK_TYPE_ID: i32 = 2;
    const AZIMUTH_TYPE_ID: i32 = 3;

    fn encode_contents<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Empty { .. } => Ok(()),
            Self::Position { position, .. } => {
                VarIntWrapper(position.x).encode(writer)?;
                VarIntWrapper(position.y).encode(writer)?;
                VarIntWrapper(position.z).encode(writer)
            }
            Self::Chunk { chunk_pos, .. } => {
                VarIntWrapper(chunk_pos.x).encode(writer)?;
                VarIntWrapper(chunk_pos.z).encode(writer)
            }
            Self::Azimuth { angle_radians, .. } => angle_radians.encode(writer),
        }
    }

    const fn type_id(&self) -> i32 {
        match self {
            Self::Empty { .. } => Self::EMPTY_TYPE_ID,
            Self::Position { .. } => Self::POSITION_TYPE_ID,
            Self::Chunk { .. } => Self::CHUNK_TYPE_ID,
            Self::Azimuth { .. } => Self::AZIMUTH_TYPE_ID,
        }
    }

    fn id(&self) -> &TrackedWaypointId {
        match self {
            Self::Empty { id, .. }
            | Self::Position { id, .. }
            | Self::Chunk { id, .. }
            | Self::Azimuth { id, .. } => id,
        }
    }

    fn icon(&self) -> &WaypointIcon {
        match self {
            Self::Empty { icon, .. }
            | Self::Position { icon, .. }
            | Self::Chunk { icon, .. }
            | Self::Azimuth { icon, .. } => icon,
        }
    }
}

impl DataType for TrackedWaypointId {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::EntityUuid(uuid) => {
                true.encode(writer)?;
                uuid.encode(writer)
            }
            Self::Named(name) => {
                false.encode(writer)?;
                name.encode(writer)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        if bool::decode(reader)? {
            return Uuid::decode(reader).map(Self::EntityUuid);
        }

        String::decode(reader).map(Self::Named)
    }
}

impl DataType for WaypointIcon {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.style_asset.encode(writer)?;
        self.color.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            style_asset: Identifier::decode(reader)?,
            color: Option::<i32>::decode(reader)?,
        })
    }
}

impl DataType for TrackedWaypoint {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id().encode(writer)?;
        self.icon().encode(writer)?;
        VarIntWrapper(self.type_id()).encode(writer)?;
        self.encode_contents(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let id = TrackedWaypointId::decode(reader)?;
        let icon = WaypointIcon::decode(reader)?;
        match VarIntWrapper::decode(reader)?.0 {
            Self::EMPTY_TYPE_ID => Ok(Self::Empty { id, icon }),
            Self::POSITION_TYPE_ID => Ok(Self::Position {
                id,
                icon,
                position: Vector3i {
                    x: VarIntWrapper::decode(reader)?.0,
                    y: VarIntWrapper::decode(reader)?.0,
                    z: VarIntWrapper::decode(reader)?.0,
                },
            }),
            Self::CHUNK_TYPE_ID => Ok(Self::Chunk {
                id,
                icon,
                chunk_pos: ChunkPos {
                    x: VarIntWrapper::decode(reader)?.0,
                    z: VarIntWrapper::decode(reader)?.0,
                },
            }),
            Self::AZIMUTH_TYPE_ID => Ok(Self::Azimuth {
                id,
                icon,
                angle_radians: f32::decode(reader)?,
            }),
            waypoint_type_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown tracked waypoint type {waypoint_type_id}"),
            )),
        }
    }
}
