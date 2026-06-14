use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use crate::types::{Position, Slot, Vector3d};
use spinel_registry::{ParticlePayloadShape, ParticleType};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub particle_type_id: i32,
    pub payload: ParticlePayload,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParticlePayload {
    Unit,
    BlockState(i32),
    ColorScale {
        color: i32,
        scale: f32,
    },
    ColorTransitionScale {
        color: i32,
        transition_color: i32,
        scale: f32,
    },
    ItemStack(Slot),
    AlphaColor(i32),
    Float(f32),
    VarInt(i32),
    Vibration(VibrationParticlePayload),
    Trail {
        target: Vector3d,
        color: i32,
        duration: i32,
    },
    ColorPower {
        color: i32,
        power: f32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum VibrationParticlePayload {
    Block {
        source_position: Position,
        travel_ticks: i32,
    },
    Entity {
        source_entity_id: i32,
        source_entity_eye_height: f32,
        travel_ticks: i32,
    },
}

impl Particle {
    pub const fn new(particle_type_id: i32, payload: ParticlePayload) -> Self {
        Self {
            particle_type_id,
            payload,
        }
    }

    pub const fn effect() -> Self {
        Self::new(
            16,
            ParticlePayload::ColorPower {
                color: 0x00ff_ffff,
                power: 1.0,
            },
        )
    }
}

impl DataType for Particle {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let particle_type = particle_type_by_id(self.particle_type_id)?;
        if !payload_matches_shape(&self.payload, particle_type.payload_shape()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Particle payload does not match {} payload shape",
                    particle_type.name()
                ),
            ));
        }

        VarIntWrapper(self.particle_type_id).encode(writer)?;
        self.payload.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let particle_type_id = VarIntWrapper::decode(reader)?.0;
        let particle_type = particle_type_by_id(particle_type_id)?;
        Ok(Self {
            particle_type_id,
            payload: ParticlePayload::decode_for_shape(reader, particle_type.payload_shape())?,
        })
    }
}

impl ParticlePayload {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Unit => Ok(()),
            Self::BlockState(block_state_id) => VarIntWrapper(*block_state_id).encode(writer),
            Self::ColorScale { color, scale } => {
                color.encode(writer)?;
                scale.encode(writer)
            }
            Self::ColorTransitionScale {
                color,
                transition_color,
                scale,
            } => {
                color.encode(writer)?;
                transition_color.encode(writer)?;
                scale.encode(writer)
            }
            Self::ItemStack(item_stack) => item_stack.encode(writer),
            Self::AlphaColor(color) => color.encode(writer),
            Self::Float(value) => value.encode(writer),
            Self::VarInt(value) => VarIntWrapper(*value).encode(writer),
            Self::Vibration(vibration) => vibration.encode(writer),
            Self::Trail {
                target,
                color,
                duration,
            } => {
                target.encode(writer)?;
                color.encode(writer)?;
                VarIntWrapper(*duration).encode(writer)
            }
            Self::ColorPower { color, power } => {
                color.encode(writer)?;
                power.encode(writer)
            }
        }
    }

    fn decode_for_shape<R: Read>(
        reader: &mut R,
        payload_shape: ParticlePayloadShape,
    ) -> io::Result<Self> {
        match payload_shape {
            ParticlePayloadShape::Unit => Ok(Self::Unit),
            ParticlePayloadShape::BlockState => {
                Ok(Self::BlockState(VarIntWrapper::decode(reader)?.0))
            }
            ParticlePayloadShape::ColorScale => Ok(Self::ColorScale {
                color: i32::decode(reader)?,
                scale: f32::decode(reader)?,
            }),
            ParticlePayloadShape::ColorTransitionScale => Ok(Self::ColorTransitionScale {
                color: i32::decode(reader)?,
                transition_color: i32::decode(reader)?,
                scale: f32::decode(reader)?,
            }),
            ParticlePayloadShape::ItemStack => Ok(Self::ItemStack(Slot::decode(reader)?)),
            ParticlePayloadShape::AlphaColor => Ok(Self::AlphaColor(i32::decode(reader)?)),
            ParticlePayloadShape::Float => Ok(Self::Float(f32::decode(reader)?)),
            ParticlePayloadShape::VarInt => Ok(Self::VarInt(VarIntWrapper::decode(reader)?.0)),
            ParticlePayloadShape::Vibration => {
                Ok(Self::Vibration(VibrationParticlePayload::decode(reader)?))
            }
            ParticlePayloadShape::Trail => Ok(Self::Trail {
                target: Vector3d::decode(reader)?,
                color: i32::decode(reader)?,
                duration: VarIntWrapper::decode(reader)?.0,
            }),
            ParticlePayloadShape::ColorPower => Ok(Self::ColorPower {
                color: i32::decode(reader)?,
                power: f32::decode(reader)?,
            }),
        }
    }
}

impl VibrationParticlePayload {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Block {
                source_position,
                travel_ticks,
            } => {
                VarIntWrapper(0).encode(writer)?;
                source_position.encode(writer)?;
                VarIntWrapper(*travel_ticks).encode(writer)
            }
            Self::Entity {
                source_entity_id,
                source_entity_eye_height,
                travel_ticks,
            } => {
                VarIntWrapper(1).encode(writer)?;
                VarIntWrapper(*source_entity_id).encode(writer)?;
                source_entity_eye_height.encode(writer)?;
                VarIntWrapper(*travel_ticks).encode(writer)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Block {
                source_position: Position::decode(reader)?,
                travel_ticks: VarIntWrapper::decode(reader)?.0,
            }),
            1 => Ok(Self::Entity {
                source_entity_id: VarIntWrapper::decode(reader)?.0,
                source_entity_eye_height: f32::decode(reader)?,
                travel_ticks: VarIntWrapper::decode(reader)?.0,
            }),
            source_type => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown vibration source type: {source_type}"),
            )),
        }
    }
}

fn particle_type_by_id(particle_type_id: i32) -> io::Result<&'static ParticleType> {
    ParticleType::from_id(particle_type_id).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown particle type id: {particle_type_id}"),
        )
    })
}

fn payload_matches_shape(payload: &ParticlePayload, payload_shape: ParticlePayloadShape) -> bool {
    matches!(
        (payload, payload_shape),
        (ParticlePayload::Unit, ParticlePayloadShape::Unit)
            | (
                ParticlePayload::BlockState(_),
                ParticlePayloadShape::BlockState
            )
            | (
                ParticlePayload::ColorScale { .. },
                ParticlePayloadShape::ColorScale
            )
            | (
                ParticlePayload::ColorTransitionScale { .. },
                ParticlePayloadShape::ColorTransitionScale
            )
            | (
                ParticlePayload::ItemStack(_),
                ParticlePayloadShape::ItemStack
            )
            | (
                ParticlePayload::AlphaColor(_),
                ParticlePayloadShape::AlphaColor
            )
            | (ParticlePayload::Float(_), ParticlePayloadShape::Float)
            | (ParticlePayload::VarInt(_), ParticlePayloadShape::VarInt)
            | (
                ParticlePayload::Vibration(_),
                ParticlePayloadShape::Vibration
            )
            | (ParticlePayload::Trail { .. }, ParticlePayloadShape::Trail)
            | (
                ParticlePayload::ColorPower { .. },
                ParticlePayloadShape::ColorPower
            )
    )
}
