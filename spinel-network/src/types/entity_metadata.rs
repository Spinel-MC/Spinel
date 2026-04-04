use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};
use uuid::Uuid;

use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use crate::types::{GlobalPos, Position, Quaternionf, Slot, Vector3f};
use crate::wrappers::JsonTextComponent;

#[derive(Debug, Clone)]
pub struct MetadataEntry {
    pub index: u8,
    pub value: MetadataValue,
}

#[derive(Debug, Clone)]
pub enum MetadataValue {
    Byte(i8),
    VarInt(i32),
    Long(i64),
    Float(f32),
    String(String),
    Text(TextComponent),
    OptionalText(Option<TextComponent>),
    Slot(Slot),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(Position),
    OptionalPosition(Option<Position>),
    Direction(i32),
    OptionalUuid(Option<Uuid>),
    BlockState(i32),
    OptionalBlockState(i32),
    Particle(Vec<u8>),
    ParticleList(Vec<u8>),
    VillagerData(i32, i32, i32),
    OptionalVarInt(Option<i32>),
    Pose(i32),
    CatVariant(i32),
    CowVariant(i32),
    WolfVariant(i32),
    WolfSoundVariant(i32),
    FrogVariant(i32),
    PigVariant(i32),
    ChickenVariant(i32),
    OptionalGlobalPos(Option<GlobalPos>),
    PaintingVariant(i32),
    SnifferState(i32),
    ArmadilloState(i32),
    CopperGolemState(i32),
    OxidationLevel(i32),
    Vector3f(Vector3f),
    Quaternionf(Quaternionf),
}

impl DataType for MetadataEntry {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.index.encode(w)?;
        self.value.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let index = u8::decode(r)?;
        if index == 0xFF {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Read 0xFF in MetadataEntry decode",
            ));
        }
        let value = MetadataValue::decode(r)?;
        Ok(MetadataEntry { index, value })
    }
}

impl DataType for MetadataValue {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            MetadataValue::Byte(v) => {
                VarIntWrapper(0).encode(w)?;
                v.encode(w)
            }
            MetadataValue::VarInt(v) => {
                VarIntWrapper(1).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::Long(v) => {
                VarIntWrapper(2).encode(w)?;
                v.encode(w)
            }
            MetadataValue::Float(v) => {
                VarIntWrapper(3).encode(w)?;
                v.encode(w)
            }
            MetadataValue::String(v) => {
                VarIntWrapper(4).encode(w)?;
                v.encode(w)
            }
            MetadataValue::Text(v) => {
                VarIntWrapper(5).encode(w)?;
                JsonTextComponent(v.clone()).encode(w)
            }
            MetadataValue::OptionalText(v) => {
                VarIntWrapper(6).encode(w)?;
                v.clone().map(JsonTextComponent).encode(w)
            }
            MetadataValue::Slot(v) => {
                VarIntWrapper(7).encode(w)?;
                v.encode(w)
            }
            MetadataValue::Boolean(v) => {
                VarIntWrapper(8).encode(w)?;
                v.encode(w)
            }
            MetadataValue::Rotation(x, y, z) => {
                VarIntWrapper(9).encode(w)?;
                x.encode(w)?;
                y.encode(w)?;
                z.encode(w)
            }
            MetadataValue::Position(v) => {
                VarIntWrapper(10).encode(w)?;
                v.encode(w)
            }
            MetadataValue::OptionalPosition(v) => {
                VarIntWrapper(11).encode(w)?;
                v.encode(w)
            }
            MetadataValue::Direction(v) => {
                VarIntWrapper(12).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::OptionalUuid(v) => {
                VarIntWrapper(13).encode(w)?;
                v.encode(w)
            }
            MetadataValue::BlockState(v) => {
                VarIntWrapper(14).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::OptionalBlockState(v) => {
                VarIntWrapper(15).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::Particle(_) => {
                VarIntWrapper(16).encode(w)?;
                Ok(())
            }
            MetadataValue::ParticleList(_) => {
                VarIntWrapper(17).encode(w)?;
                Ok(())
            }
            MetadataValue::VillagerData(t, p, l) => {
                VarIntWrapper(18).encode(w)?;
                VarIntWrapper(*t).encode(w)?;
                VarIntWrapper(*p).encode(w)?;
                VarIntWrapper(*l).encode(w)
            }
            MetadataValue::OptionalVarInt(v) => {
                VarIntWrapper(19).encode(w)?;
                v.map(VarIntWrapper).encode(w)
            }
            MetadataValue::Pose(v) => {
                VarIntWrapper(20).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::CatVariant(v) => {
                VarIntWrapper(21).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::CowVariant(v) => {
                VarIntWrapper(22).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::WolfVariant(v) => {
                VarIntWrapper(23).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::WolfSoundVariant(v) => {
                VarIntWrapper(24).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::FrogVariant(v) => {
                VarIntWrapper(25).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::PigVariant(v) => {
                VarIntWrapper(26).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::ChickenVariant(v) => {
                VarIntWrapper(27).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::OptionalGlobalPos(v) => {
                VarIntWrapper(28).encode(w)?;
                v.encode(w)
            }
            MetadataValue::PaintingVariant(v) => {
                VarIntWrapper(29).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::SnifferState(v) => {
                VarIntWrapper(30).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::ArmadilloState(v) => {
                VarIntWrapper(31).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::CopperGolemState(v) => {
                VarIntWrapper(32).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::OxidationLevel(v) => {
                VarIntWrapper(33).encode(w)?;
                VarIntWrapper(*v).encode(w)
            }
            MetadataValue::Vector3f(v) => {
                VarIntWrapper(34).encode(w)?;
                v.encode(w)
            }
            MetadataValue::Quaternionf(v) => {
                VarIntWrapper(35).encode(w)?;
                v.encode(w)
            }
        }
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let type_id = VarIntWrapper::decode(r)?.0;
        match type_id {
            0 => Ok(MetadataValue::Byte(i8::decode(r)?)),
            1 => Ok(MetadataValue::VarInt(VarIntWrapper::decode(r)?.0)),
            2 => Ok(MetadataValue::Long(i64::decode(r)?)),
            3 => Ok(MetadataValue::Float(f32::decode(r)?)),
            4 => Ok(MetadataValue::String(String::decode(r)?)),
            5 => Ok(MetadataValue::Text(JsonTextComponent::decode(r)?.0)),
            6 => Ok(MetadataValue::OptionalText(
                Option::<JsonTextComponent>::decode(r)?.map(|w| w.0),
            )),
            7 => Ok(MetadataValue::Slot(Slot::decode(r)?)),
            8 => Ok(MetadataValue::Boolean(bool::decode(r)?)),
            9 => Ok(MetadataValue::Rotation(
                f32::decode(r)?,
                f32::decode(r)?,
                f32::decode(r)?,
            )),
            10 => Ok(MetadataValue::Position(Position::decode(r)?)),
            11 => Ok(MetadataValue::OptionalPosition(Option::<Position>::decode(
                r,
            )?)),
            12 => Ok(MetadataValue::Direction(VarIntWrapper::decode(r)?.0)),
            13 => Ok(MetadataValue::OptionalUuid(Option::<Uuid>::decode(r)?)),
            14 => Ok(MetadataValue::BlockState(VarIntWrapper::decode(r)?.0)),
            15 => Ok(MetadataValue::OptionalBlockState(
                VarIntWrapper::decode(r)?.0,
            )),
            16 => Ok(MetadataValue::Particle(vec![])),
            17 => Ok(MetadataValue::ParticleList(vec![])),
            18 => Ok(MetadataValue::VillagerData(
                VarIntWrapper::decode(r)?.0,
                VarIntWrapper::decode(r)?.0,
                VarIntWrapper::decode(r)?.0,
            )),
            19 => Ok(MetadataValue::OptionalVarInt(
                Option::<VarIntWrapper>::decode(r)?.map(|v| v.0),
            )),
            20 => Ok(MetadataValue::Pose(VarIntWrapper::decode(r)?.0)),
            21 => Ok(MetadataValue::CatVariant(VarIntWrapper::decode(r)?.0)),
            22 => Ok(MetadataValue::CowVariant(VarIntWrapper::decode(r)?.0)),
            23 => Ok(MetadataValue::WolfVariant(VarIntWrapper::decode(r)?.0)),
            24 => Ok(MetadataValue::WolfSoundVariant(VarIntWrapper::decode(r)?.0)),
            25 => Ok(MetadataValue::FrogVariant(VarIntWrapper::decode(r)?.0)),
            26 => Ok(MetadataValue::PigVariant(VarIntWrapper::decode(r)?.0)),
            27 => Ok(MetadataValue::ChickenVariant(VarIntWrapper::decode(r)?.0)),
            28 => Ok(MetadataValue::OptionalGlobalPos(
                Option::<GlobalPos>::decode(r)?,
            )),
            29 => Ok(MetadataValue::PaintingVariant(VarIntWrapper::decode(r)?.0)),
            30 => Ok(MetadataValue::SnifferState(VarIntWrapper::decode(r)?.0)),
            31 => Ok(MetadataValue::ArmadilloState(VarIntWrapper::decode(r)?.0)),
            32 => Ok(MetadataValue::CopperGolemState(VarIntWrapper::decode(r)?.0)),
            33 => Ok(MetadataValue::OxidationLevel(VarIntWrapper::decode(r)?.0)),
            34 => Ok(MetadataValue::Vector3f(Vector3f::decode(r)?)),
            35 => Ok(MetadataValue::Quaternionf(Quaternionf::decode(r)?)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown metadata type: {}", type_id),
            )),
        }
    }
}
