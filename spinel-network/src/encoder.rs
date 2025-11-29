use spinel_nbt::{Nbt, NbtCompound, to_bytes_unnamed};
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

use crate::types::{
    chat::ChatType,
    chunk::{ChunkData, PalettedContainer},
    entity_metadata::{MetadataEntry, MetadataValue},
    game_profile::GameProfile,
    light::LightData,
    position::Position,
    slot::Slot,
    sound::SoundEvent,
    teleport_flags::TeleportFlags,
};

#[derive(Default)]
pub struct NetworkBuffer {
    pub buffer: Vec<u8>,
}

impl NetworkBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }

    pub fn write_bool(&mut self, value: bool) -> &mut Self {
        self.buffer.push(if value { 0x01 } else { 0x00 });
        self
    }

    pub fn write_byte(&mut self, value: i8) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_unsigned_byte(&mut self, value: u8) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_short(&mut self, value: i16) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_unsigned_short(&mut self, value: u16) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_int(&mut self, value: i32) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_long(&mut self, value: i64) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_u64(&mut self, value: u64) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_float(&mut self, value: f32) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_double(&mut self, value: f64) -> &mut Self {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self
    }

    pub fn write_varint(&mut self, value: i32) -> &mut Self {
        let mut uvalue = value as u32;
        loop {
            if (uvalue & !0x7F) == 0 {
                self.buffer.push(uvalue as u8);
                return self;
            }
            self.buffer.push(((uvalue & 0x7F) | 0x80) as u8);
            uvalue >>= 7;
        }
    }

    pub fn write_nbt_compound(&mut self, value: &NbtCompound) -> &mut Self {
        let mut data_buf = Vec::new();
        to_bytes_unnamed(value, &mut data_buf).expect("Failed to serialize NBT compound");
        self.buffer.extend_from_slice(&data_buf);
        self
    }

    pub fn write_game_profile(&mut self, profile: &GameProfile) -> &mut Self {
        self.write_uuid(&profile.uuid);
        self.write_string(&profile.username);
        self.write_varint(profile.properties.len() as i32);
        for prop in &profile.properties {
            self.write_string(&prop.name);
            self.write_string(&prop.value);
            self.write_bool(prop.signature.is_some());
            if let Some(sig) = &prop.signature {
                self.write_string(sig);
            }
        }
        self
    }

    pub fn write_varlong(&mut self, value: i64) -> &mut Self {
        let mut uvalue = value as u64;
        loop {
            if (uvalue & !0x7F) == 0 {
                self.buffer.push(uvalue as u8);
                return self;
            }
            self.buffer.push(((uvalue & 0x7F) | 0x80) as u8);
            uvalue >>= 7;
        }
    }

    pub fn write_string(&mut self, value: &str) -> &mut Self {
        self.write_varint(value.len() as i32);
        self.buffer.extend_from_slice(value.as_bytes());
        self
    }

    pub fn write_json_text_component(&mut self, value: &TextComponent) -> &mut Self {
        let json_string = value.to_json_string();
        self.write_string(&json_string)
    }

    pub fn write_nbt_text_component(&mut self, value: &TextComponent) -> &mut Self {
        let nbt = value.to_nbt_compound();
        self.write_nbt_compound(&nbt)
    }

    pub fn write_position(&mut self, value: &Position) -> &mut Self {
        let val = ((value.x as u64 & 0x3FFFFFF) << 38)
            | ((value.z as u64 & 0x3FFFFFF) << 12)
            | (value.y as u64 & 0xFFF);
        self.buffer.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_angle(&mut self, value: u8) -> &mut Self {
        self.write_unsigned_byte(value)
    }

    pub fn write_byte_array(&mut self, value: &[u8]) -> &mut Self {
        self.write_varint(value.len() as i32);
        self.buffer.extend_from_slice(value);
        self
    }

    pub fn extend_from_slice(&mut self, slice: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(slice);
        self
    }

    pub fn write_bitset(&mut self, value: &[i64]) -> &mut Self {
        self.write_varint(value.len() as i32);
        for &long in value {
            self.write_long(long);
        }
        self
    }

    pub fn write_fixed_bitset(&mut self, value: &[i64]) -> &mut Self {
        for &long in value {
            self.write_long(long);
        }
        self
    }

    pub fn write_slot(&mut self, value: &Slot) -> &mut Self {
        self.write_bool(value.is_present);
        if value.is_present {
            self.write_varint(value.item_id);
            self.write_byte(value.item_count);
            if let Some(nbt) = &value.nbt {
                Nbt::Compound(nbt.clone())
                    .write_unnamed(&mut self.buffer)
                    .unwrap();
            } else {
                self.buffer.push(0);
            }
        }
        self
    }

    pub fn write_entity_metadata(&mut self, metadata: &[MetadataEntry]) -> &mut Self {
        for entry in metadata {
            self.write_unsigned_byte(entry.index);
            match &entry.value {
                MetadataValue::Byte(val) => {
                    self.write_varint(0);
                    self.write_byte(*val);
                }
                MetadataValue::VarInt(val) => {
                    self.write_varint(1);
                    self.write_varint(*val);
                }
                MetadataValue::VarLong(val) => {
                    self.write_varint(2);
                    self.write_varlong(*val);
                }
                MetadataValue::Float(val) => {
                    self.write_varint(3);
                    self.write_float(*val);
                }
                MetadataValue::String(val) => {
                    self.write_varint(4);
                    self.write_string(val);
                }
                MetadataValue::TextComponent(val) => {
                    self.write_varint(5);
                    self.write_json_text_component(val);
                }
                MetadataValue::OptionalTextComponent(val) => {
                    self.write_varint(6);
                    self.write_bool(val.is_some());
                    if let Some(v) = val {
                        self.write_json_text_component(v);
                    }
                }
                MetadataValue::Slot(val) => {
                    self.write_varint(7);
                    self.write_slot(val);
                }
                MetadataValue::Boolean(val) => {
                    self.write_varint(8);
                    self.write_bool(*val);
                }
                MetadataValue::Rotation(x, y, z) => {
                    self.write_varint(9);
                    self.write_float(*x);
                    self.write_float(*y);
                    self.write_float(*z);
                }
                MetadataValue::Position(val) => {
                    self.write_varint(10);
                    self.write_position(val);
                }
                MetadataValue::OptionalPosition(val) => {
                    self.write_varint(11);
                    self.write_bool(val.is_some());
                    if let Some(v) = val {
                        self.write_position(v);
                    }
                }
                MetadataValue::Direction(val) => {
                    self.write_varint(12);
                    self.write_varint(*val);
                }
                MetadataValue::OptionalUuid(val) => {
                    self.write_varint(13);
                    self.write_bool(val.is_some());
                    if let Some(v) = val {
                        self.buffer.extend_from_slice(v.as_bytes());
                    }
                }
                MetadataValue::BlockId(val) => {
                    self.write_varint(14);
                    self.write_varint(*val);
                }
                MetadataValue::OptionalBlockId(val) => {
                    self.write_varint(15);
                    self.write_bool(val.is_some());
                    if let Some(v) = val {
                        self.write_varint(*v);
                    }
                }
                MetadataValue::Nbt(val) => {
                    self.write_varint(16);
                    Nbt::Compound(val.clone())
                        .write_unnamed(&mut self.buffer)
                        .unwrap();
                }
            }
        }
        self.write_unsigned_byte(0xFF);
        self
    }

    pub fn write_paletted_container(&mut self, container: &PalettedContainer) -> &mut Self {
        self.write_unsigned_byte(container.bits_per_entry);
        if container.bits_per_entry > 0 && container.bits_per_entry <= 8 {
            if let Some(palette) = &container.palette {
                self.write_varint(palette.len() as i32);
                for &entry in palette {
                    self.write_varint(entry);
                }
            } else {
                self.write_varint(0);
            }
        } else if container.bits_per_entry == 0 {
            self.write_varint(
                container
                    .palette
                    .as_ref()
                    .and_then(|p| p.first().copied())
                    .unwrap_or(0),
            );
        }

        self.write_varint(container.data.len() as i32);
        for &long in &container.data {
            self.write_u64(long);
        }
        self
    }

    pub fn write_chunk_data(&mut self, data: &ChunkData) -> &mut Self {
        Nbt::Compound(data.heightmaps.clone())
            .write("Heightmaps", &mut self.buffer)
            .unwrap();

        let mut section_buffer = NetworkBuffer::new();
        for section in &data.sections {
            section_buffer.write_short(section.block_count);
            section_buffer.write_paletted_container(&section.block_states);
            section_buffer.write_paletted_container(&section.biomes);
        }

        self.write_byte_array(&section_buffer.into_buffer());

        self.write_varint(data.block_entities.len() as i32);
        for entity in &data.block_entities {
            Nbt::Compound(entity.clone())
                .write_unnamed(&mut self.buffer)
                .unwrap();
        }
        self
    }

    pub fn write_light_data(&mut self, data: &LightData) -> &mut Self {
        self.write_bitset(&data.sky_light_mask);
        self.write_bitset(&data.block_light_mask);
        self.write_bitset(&data.empty_sky_light_mask);
        self.write_bitset(&data.empty_block_light_mask);

        self.write_varint(data.sky_light_arrays.len() as i32);
        for array in &data.sky_light_arrays {
            self.write_byte_array(array);
        }

        self.write_varint(data.block_light_arrays.len() as i32);
        for array in &data.block_light_arrays {
            self.write_byte_array(array);
        }
        self
    }

    pub fn write_teleport_flags(&mut self, flags: &TeleportFlags) -> &mut Self {
        self.write_byte(flags.to_byte());
        self
    }

    pub fn write_chat_type(&mut self, chat_type: &ChatType) -> &mut Self {
        self.write_varint(chat_type.type_id);
        self.write_json_text_component(&chat_type.name);
        self.write_bool(chat_type.target.is_some());
        if let Some(target) = &chat_type.target {
            self.write_json_text_component(target);
        }
        self
    }

    pub fn write_sound_event(&mut self, sound: &SoundEvent) -> &mut Self {
        match sound {
            SoundEvent::Id(id) => {
                self.write_varint(*id + 1);
            }
            SoundEvent::Named { name, fixed_range } => {
                self.write_varint(0);
                self.write_string(name);
                self.write_bool(fixed_range.is_some());
                if let Some(range) = fixed_range {
                    self.write_float(*range);
                }
            }
        }
        self
    }

    pub fn write_uuid(&mut self, value: &Uuid) -> &mut Self {
        self.buffer.extend_from_slice(value.as_bytes());
        self
    }
}
