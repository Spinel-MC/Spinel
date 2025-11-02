use std::io::{Cursor, Error, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream, Shutdown};
use std::str::FromStr;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

use crate::client::metadata::LoginMetadata;
use crate::encoder::NetworkBuffer;

use crate::types::chat::ChatType;
use crate::types::chunk::{ChunkData, ChunkSection, PalettedContainer};
use crate::types::entity_metadata::{MetadataEntry, MetadataValue};
use crate::types::light::LightData;
use crate::types::position::Position;
use crate::types::slot::Slot;
use crate::types::sound::SoundEvent;
use crate::types::teleport_flags::TeleportFlags;
use cfb8::{Encryptor, Decryptor};
use aes::Aes128;
use cfb8::cipher::{
    KeyIvInit,
    generic_array::{GenericArray, typenum::U1},
    BlockEncryptMut, BlockDecryptMut
};

type AesEncryptor = Encryptor<Aes128>;
type AesDecryptor = Decryptor<Aes128>;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConnectionState {
    Handshaking, Status, Login, Configuration, Play
}

impl FromStr for ConnectionState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Handshaking" => Ok(ConnectionState::Handshaking),
            "Status" => Ok(ConnectionState::Status),
            "Login" => Ok(ConnectionState::Login),
            "Configuration" => Ok(ConnectionState::Configuration),
            "Play" => Ok(ConnectionState::Play),
            _ => Err(()),
        }
    }
}

impl ConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionState::Handshaking => "Handshaking",
            ConnectionState::Status => "Status",
            ConnectionState::Login => "Login",
            ConnectionState::Configuration => "Configuration",
            ConnectionState::Play => "Play",
        }
    }
}

pub struct Client {
    pub stream: TcpStream,
    pub addr: SocketAddr,
    pub state: ConnectionState,
    encryptor: Option<AesEncryptor>,
    decryptor: Option<AesDecryptor>,
    pub payload_cursor: Option<Cursor<Vec<u8>>>,
    pub login_metadata: Option<LoginMetadata>
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Client {
        Client {
            stream,
            addr,
            state: ConnectionState::Handshaking,
            encryptor: None,
            decryptor: None,
            payload_cursor: None,
            login_metadata: None,
        }
    }
    
    pub fn read_varint_from_cursor<R: Read>(reader: &mut R) -> Result<i32, Error> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf)?;
            let read = buf[0];
            let value = (read & 0x7F) as i32;
            result |= value << (7 * num_read);
            if (read & 0x80) == 0 { break; }
            num_read += 1;
            if num_read >= 5 { return Err(Error::new(ErrorKind::InvalidData, "VarInt is too big")); }
        }
        Ok(result)
    }

    pub fn read_string_from_cursor<R: Read>(reader: &mut R) -> Result<String, Error> {
        let length = Self::read_varint_from_cursor(reader)? as usize;
        if length > 32767 {
             return Err(Error::new(ErrorKind::InvalidData, "String too long"));
        }
        let mut buffer = vec![0; length];
        reader.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn read_uuid_from_cursor<R: Read>(reader: &mut R) -> Result<Uuid, Error> {
        let mut buf = [0; 16];
        reader.read_exact(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }

    pub fn enable_encryption(&mut self, shared_secret: &[u8]) {
        let key = GenericArray::from_slice(shared_secret);
        let iv = GenericArray::from_slice(shared_secret);
        self.encryptor = Some(AesEncryptor::new(key, iv));
        self.decryptor = Some(AesDecryptor::new(key, iv));
        println!("Encryption enabled for client {}", self.addr);
    }

    pub fn shutdown_write(&mut self) {
        if let Err(e) = self.stream.shutdown(Shutdown::Write) {
            eprintln!("Failed to shutdown stream for client at {}: {}", self.addr, e);
        }
    }

    pub fn disconnect(&mut self) {
        if let Err(e) = self.stream.shutdown(Shutdown::Both) {
            eprintln!("Failed to disconnect stream for client at {}: {}", self.addr, e);
        }
    }

    pub fn send_packet(&mut self, packet_id: i32, payload: &[u8]) {
        let mut buffer = NetworkBuffer::new();
        buffer.write_varint(packet_id);
        buffer.extend_from_slice(payload);
        let payload_bytes = buffer.into_buffer();

        let mut framed_buffer = NetworkBuffer::new();
        framed_buffer.write_varint(payload_bytes.len() as i32);
        framed_buffer.extend_from_slice(&payload_bytes);

        let mut final_bytes = framed_buffer.into_buffer();

        println!(
            "-> OUT to [{}]: State={:?}, ID={:#04X}, Size={}, Encrypted={}",
            self.addr,
            self.state,
            packet_id,
            final_bytes.len(),
            self.encryptor.is_some()
        );

        if let Some(encryptor) = self.encryptor.as_mut() {
            for byte in &mut final_bytes {
                let mut block = GenericArray::<u8, U1>::from_mut_slice(std::slice::from_mut(byte));
                encryptor.encrypt_block_mut(&mut block);
            }
        }

        if self.stream.write_all(&final_bytes).is_err() {
            eprintln!("Packet Response Error for client at {}", self.addr);
        }
    }


    pub fn send_raw_bytes(&mut self, bytes: &[u8]) {
        if self.stream.write_all(bytes).is_err() {
            eprintln!("Raw Byte Response Error for client at {}", self.addr);
        }
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if let Some(cursor) = self.payload_cursor.as_mut() {
            return cursor.read_exact(buf);
        }

        self.stream.read_exact(buf)?;

        if let Some(decryptor) = self.decryptor.as_mut() {
            for byte in &mut *buf {
                let mut block = GenericArray::<u8, U1>::from_mut_slice(std::slice::from_mut(byte));
                decryptor.decrypt_block_mut(&mut block);
            }
        }

        Ok(())
    }

    pub fn read_string_with_limit(&mut self, max_len: usize) -> Result<String, Error> {
        let length = self.read_varint()? as usize;

        if length > max_len {
            let msg = format!("String of length {} exceeded its limit of {}", length, max_len);
            return Err(Error::new(ErrorKind::InvalidData, msg))
        }

        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn read_string(&mut self) -> Result<String, Error> {
        self.read_string_with_limit(32767)
    }

    pub fn read_identifier(&mut self) -> Result<String, Error> {
        self.read_string_with_limit(32767)
    }

    pub fn read_bool(&mut self) -> Result<bool, Error> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }

    pub fn read_byte(&mut self) -> Result<i8, Error> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(i8::from_be_bytes(buf))
    }

    pub fn read_unsigned_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_short(&mut self) -> Result<i16, Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_int(&mut self) -> Result<i32, Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn read_long(&mut self) -> Result<i64, Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_float(&mut self) -> Result<f32, Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    pub fn read_double(&mut self) -> Result<f64, Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    pub fn read_varint(&mut self) -> Result<i32, Error> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            let mut buf = [0; 1];
            self.read_exact(&mut buf)?;
            let read = buf[0];

            let value = (read & 0x7F) as i32;
            result |= value << (7 * num_read);

            if (read & 0x80) == 0 {
                break;
            }

            num_read += 1;

            if num_read >= 5 {
                return Err(Error::new(ErrorKind::InvalidData, "VarInt is too big"));
            }
        }
        Ok(result)
    }

    pub fn read_varlong(&mut self) -> Result<i64, Error> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        loop {
            let mut buf = [0; 1];
            self.read_exact(&mut buf)?;
            let read = buf[0];

            let value = (read & 0x7F) as i64;
            result |= value << (7 * num_read);

            if (read & 0x80) == 0 {
                break;
            }

            num_read += 1;

            if num_read >= 10 {
                return Err(Error::new(ErrorKind::InvalidData, "VarLong is too big"));
            }
        }
        Ok(result)
    }

    pub fn read_json_text_component(&mut self) -> Result<TextComponent, Error> {
        let json_str = self.read_string_with_limit(262144)?;
        serde_json::from_str(&json_str)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_uuid(&mut self) -> Result<Uuid, Error> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(Uuid::from_bytes(buf))
    }

    pub fn read_position(&mut self) -> Result<Position, Error> {
        let val = self.read_long()? as u64;
        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = (val << 26 >> 38) as i32;

        let y = if y >= 2048 { y - 4096 } else { y };

        Ok(Position { x, y, z })
    }

    pub fn read_angle(&mut self) -> Result<u8, Error> {
        self.read_unsigned_byte()
    }

    pub fn read_byte_array(&mut self) -> Result<Vec<u8>, Error> {
        let length = self.read_varint()? as usize;
        let mut buf = vec![0u8; length];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_bitset(&mut self) -> Result<Vec<i64>, Error> {
        let length = self.read_varint()? as usize;
        let mut longs = Vec::with_capacity(length);
        for _ in 0..length {
            longs.push(self.read_long()?);
        }
        Ok(longs)
    }

    pub fn read_fixed_bitset(&mut self, long_count: usize) -> Result<Vec<i64>, Error> {
        let mut longs = Vec::with_capacity(long_count);
        for _ in 0..long_count {
            longs.push(self.read_long()?);
        }
        Ok(longs)
    }

    pub fn read_slot(&mut self) -> Result<Slot, Error> {
        let is_present = self.read_bool()?;
        if !is_present {
            return Ok(Slot { is_present: false, item_id: 0, item_count: 0, nbt: None });
        }
        
        let item_id = self.read_varint()?;
        let item_count = self.read_byte()?;

        let mut tag_id_buf = [0; 1];
        self.stream.peek(&mut tag_id_buf)?;

        let nbt = if tag_id_buf[0] == 0 {
            self.read_byte()?;
            None
        } else {
            let tag_id = self.read_unsigned_byte()?;
            if tag_id != spinel_nbt::COMPOUND_ID {
                return Err(Error::new(ErrorKind::InvalidData, "Slot NBT must be a Compound tag"));
            }
            let mut helper = spinel_nbt::deserializer::NbtReadHelper::new(&mut self.stream);
            let compound = NbtCompound::deserialize_content(&mut helper).map_err(|e| Error::new(ErrorKind::Other, e))?;
            Some(compound)
        };

        Ok(Slot { is_present: true, item_id, item_count, nbt })
    }

    pub fn read_entity_metadata(&mut self) -> Result<Vec<MetadataEntry>, Error> {
        let mut metadata = Vec::new();
        loop {
            let index = self.read_unsigned_byte()?;
            if index == 0xFF {
                break;
            }
            let type_id = self.read_varint()?;
            let value = match type_id {
                0 => MetadataValue::Byte(self.read_byte()?),
                1 => MetadataValue::VarInt(self.read_varint()?),
                2 => MetadataValue::VarLong(self.read_varlong()?),
                3 => MetadataValue::Float(self.read_float()?),
                4 => MetadataValue::String(self.read_string()?),
                5 => MetadataValue::TextComponent(self.read_json_text_component()?),
                6 => {
                    if self.read_bool()? {
                        MetadataValue::OptionalTextComponent(Some(self.read_json_text_component()?))
                    } else {
                        MetadataValue::OptionalTextComponent(None)
                    }
                }
                7 => MetadataValue::Slot(self.read_slot()?),
                8 => MetadataValue::Boolean(self.read_bool()?),
                9 => MetadataValue::Rotation(self.read_float()?, self.read_float()?, self.read_float()?),
                10 => MetadataValue::Position(self.read_position()?),
                11 => {
                    if self.read_bool()? {
                        MetadataValue::OptionalPosition(Some(self.read_position()?))
                    } else {
                        MetadataValue::OptionalPosition(None)
                    }
                },
                12 => MetadataValue::Direction(self.read_varint()?),
                13 => {
                    if self.read_bool()? {
                        MetadataValue::OptionalUuid(Some(self.read_uuid()?))
                    } else {
                        MetadataValue::OptionalUuid(None)
                    }
                },
                14 => MetadataValue::BlockId(self.read_varint()?),
                15 => {
                     if self.read_bool()? {
                        MetadataValue::OptionalBlockId(Some(self.read_varint()?))
                    } else {
                        MetadataValue::OptionalBlockId(None)
                    }
                },
                16 => {
                    let tag_id = self.read_unsigned_byte()?;
                    if tag_id != spinel_nbt::COMPOUND_ID {
                        return Err(Error::new(ErrorKind::InvalidData, "Metadata NBT must be a Compound tag"));
                    }
                    let mut helper = spinel_nbt::deserializer::NbtReadHelper::new(&mut self.stream);
                    let compound = NbtCompound::deserialize_content(&mut helper).map_err(|e| Error::new(ErrorKind::Other, e))?;
                    MetadataValue::Nbt(compound)
                }
                _ => return Err(Error::new(ErrorKind::InvalidData, format!("Unknown metadata type ID: {}", type_id))),
            };
            metadata.push(MetadataEntry { index, value });
        }
        Ok(metadata)
    }

    fn cursor_read_exact<R: Read>(reader: &mut R, buf: &mut [u8]) -> Result<(), Error> {
        reader.read_exact(buf)
    }

    fn cursor_read_varint<R: Read>(reader: &mut R) -> Result<i32, Error> {
        let mut num_read = 0;
        let mut result = 0;
        loop {
            let mut buf = [0; 1];
            Self::cursor_read_exact(reader, &mut buf)?;
            let read = buf[0];
            let value = (read & 0x7F) as i32;
            result |= value << (7 * num_read);
            if (read & 0x80) == 0 { break; }
            num_read += 1;
            if num_read >= 5 { return Err(Error::new(ErrorKind::InvalidData, "VarInt too big")); }
        }
        Ok(result)
    }

    fn cursor_read_short<R: Read>(reader: &mut R) -> Result<i16, Error> {
        let mut buf = [0; 2];
        Self::cursor_read_exact(reader, &mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn cursor_read_u64<R: Read>(reader: &mut R) -> Result<u64, Error> {
        let mut buf = [0; 8];
        Self::cursor_read_exact(reader, &mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }


    fn read_paletted_container_from_cursor<R: Read>(reader: &mut R) -> Result<PalettedContainer, Error> {
        let bits_per_entry = {
            let mut buf = [0; 1];
            Self::cursor_read_exact(reader, &mut buf)?;
            buf[0]
        };

        let palette = if bits_per_entry == 0 {
            let single_value = Self::cursor_read_varint(reader)?;
            Some(vec![single_value])
        } else if bits_per_entry <= 8 {
            let palette_size = Self::cursor_read_varint(reader)? as usize;
            let mut palette_vec = Vec::with_capacity(palette_size);
            for _ in 0..palette_size {
                palette_vec.push(Self::cursor_read_varint(reader)?);
            }
            Some(palette_vec)
        } else {
            None
        };

        let data_array_len = Self::cursor_read_varint(reader)? as usize;
        let mut data = Vec::with_capacity(data_array_len);
        for _ in 0..data_array_len {
            data.push(Self::cursor_read_u64(reader)?);
        }

        Ok(PalettedContainer { bits_per_entry, palette, data })
    }

    fn read_chunk_section_from_cursor<R: Read>(reader: &mut R) -> Result<ChunkSection, Error> {
        let block_count = Self::cursor_read_short(reader)?;
        let block_states = Self::read_paletted_container_from_cursor(reader)?;
        let biomes = Self::read_paletted_container_from_cursor(reader)?;
        Ok(ChunkSection { block_count, block_states, biomes })
    }

    pub fn read_chunk_data(&mut self) -> Result<ChunkData, Error> {
        let heightmaps = if let Nbt::Compound(c) = Nbt::read_from_stream(&mut self.stream)?.1 { c } else { return Err(Error::new(ErrorKind::InvalidData, "Expected NBT Compound for Heightmaps")); };
        let data_blob = self.read_byte_array()?;
        let num_block_entities = self.read_varint()? as usize;
        let mut block_entities = Vec::with_capacity(num_block_entities);
        for _ in 0..num_block_entities {
            let tag_id = self.read_unsigned_byte()?;
            if tag_id != spinel_nbt::COMPOUND_ID {
                 return Err(Error::new(ErrorKind::InvalidData, "Expected NBT Compound for Block Entity"));
            }
            let mut helper = spinel_nbt::deserializer::NbtReadHelper::new(&mut self.stream);
            let compound = NbtCompound::deserialize_content(&mut helper).map_err(|e| Error::new(ErrorKind::Other, e))?;
            block_entities.push(compound)
        }

        let mut sections = Vec::new();
        let mut data_cursor = Cursor::new(data_blob);

        while (data_cursor.position() as usize) < data_cursor.get_ref().len() {
            let section = Self::read_chunk_section_from_cursor(&mut data_cursor)?;
            sections.push(section);
        }

        Ok(ChunkData { heightmaps, sections, block_entities })
    }

    pub fn read_light_data(&mut self) -> Result<LightData, Error> {
        let sky_light_mask = self.read_bitset()?;
        let block_light_mask = self.read_bitset()?;
        let empty_sky_light_mask = self.read_bitset()?;
        let empty_block_light_mask = self.read_bitset()?;

        let sky_light_arrays_count = self.read_varint()? as usize;
        let mut sky_light_arrays = Vec::with_capacity(sky_light_arrays_count);
        for _ in 0..sky_light_arrays_count {
            sky_light_arrays.push(self.read_byte_array()?);
        }

        let block_light_arrays_count = self.read_varint()? as usize;
        let mut block_light_arrays = Vec::with_capacity(block_light_arrays_count);
        for _ in 0..block_light_arrays_count {
            block_light_arrays.push(self.read_byte_array()?);
        }

        Ok(LightData {
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light_arrays,
            block_light_arrays,
        })
    }

    pub fn read_teleport_flags(&mut self) -> Result<TeleportFlags, Error> {
        Ok(TeleportFlags::from_byte(self.read_byte()?))
    }

    pub fn read_chat_type(&mut self) -> Result<ChatType, Error> {
        let type_id = self.read_varint()?;
        let name = self.read_json_text_component()?;
        let has_target = self.read_bool()?;
        let target = if has_target { Some(self.read_json_text_component()?) } else { None };
        Ok(ChatType { type_id, name, target })
    }

    pub fn read_sound_event(&mut self) -> Result<SoundEvent, Error> {
        let id = self.read_varint()?;
        if id == 0 {
            let name = self.read_string()?;
            let has_fixed_range = self.read_bool()?;
            let fixed_range = if has_fixed_range { Some(self.read_float()?) } else { None };
            Ok(SoundEvent::Named { name, fixed_range })
        } else {
            Ok(SoundEvent::Id(id - 1))
        }
    }
}