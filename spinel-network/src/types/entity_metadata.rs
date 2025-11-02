use spinel_nbt::NbtCompound;
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

use crate::types::{position::Position, slot::Slot};

#[derive(Debug, Clone)]
pub struct MetadataEntry {
    pub index: u8,
    pub value: MetadataValue,
}

#[derive(Debug, Clone)]
pub enum MetadataValue {
    Byte(i8),
    VarInt(i32),
    VarLong(i64),
    Float(f32),
    String(String),
    TextComponent(TextComponent),
    OptionalTextComponent(Option<TextComponent>),
    Slot(Slot),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(Position),
    OptionalPosition(Option<Position>),
    Direction(i32),
    OptionalUuid(Option<Uuid>),
    BlockId(i32),
    OptionalBlockId(Option<i32>),
    Nbt(NbtCompound),
}