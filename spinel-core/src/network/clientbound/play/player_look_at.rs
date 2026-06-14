use spinel_network::data_type::DataType;
use spinel_network::types::Vector3d;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

pub struct PlayerLookAtPacket {
    pub look_at: PlayerLookAt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacePoint {
    Feet,
    Eyes,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerLookAt {
    pub face_point: FacePoint,
    pub target: Vector3d,
    pub entity: Option<PlayerLookAtEntity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerLookAtEntity {
    pub entity_id: i32,
    pub target_point: FacePoint,
}

impl PlayerLookAtPacket {
    pub const fn get_id() -> i32 {
        0x45
    }

    pub fn at_position(face_point: FacePoint, target: Vector3d) -> Self {
        Self {
            look_at: PlayerLookAt {
                face_point,
                target,
                entity: None,
            },
        }
    }

    pub fn at_entity(
        face_point: FacePoint,
        target: Vector3d,
        entity_id: i32,
        target_point: FacePoint,
    ) -> Self {
        Self {
            look_at: PlayerLookAt {
                face_point,
                target,
                entity: Some(PlayerLookAtEntity {
                    entity_id,
                    target_point,
                }),
            },
        }
    }
}

impl DataType for FacePoint {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(match self {
            Self::Feet => 0,
            Self::Eyes => 1,
        })
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Feet),
            1 => Ok(Self::Eyes),
            face_point => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown face point: {face_point}"),
            )),
        }
    }
}

impl DataType for PlayerLookAt {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.face_point.encode(writer)?;
        self.target.encode(writer)?;
        self.entity.is_some().encode(writer)?;
        if let Some(entity) = self.entity {
            VarIntWrapper(entity.entity_id).encode(writer)?;
            entity.target_point.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let face_point = FacePoint::decode(reader)?;
        let target = Vector3d::decode(reader)?;
        let entity = match bool::decode(reader)? {
            true => Some(PlayerLookAtEntity {
                entity_id: VarIntWrapper::decode(reader)?.0,
                target_point: FacePoint::decode(reader)?,
            }),
            false => None,
        };
        Ok(Self {
            face_point,
            target,
            entity,
        })
    }
}

impl DataType for PlayerLookAtPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.look_at.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            look_at: PlayerLookAt::decode(reader)?,
        })
    }
}

impl PacketStruct for PlayerLookAtPacket {
    fn get_id() -> i32 {
        0x45
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(PlayerLookAtPacket, spinel_network::Recipient::Client);
