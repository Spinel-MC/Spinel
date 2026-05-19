use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use spinel_nbt::NbtCompound;
use spinel_registry::block_entity_type::BlockEntityType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct PalettedContainer {
    pub bits_per_entry: u8,
    pub palette: Option<Vec<i32>>,
    pub data: Vec<u64>,
}

impl DataType for PalettedContainer {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.bits_per_entry.encode(w)?;

        if self.bits_per_entry == 0 {
            let val = match self.palette.as_ref().and_then(|p| p.first()).copied() {
                Some(palette_entry) => palette_entry,
                None => 0,
            };
            VarIntWrapper(val).encode(w)?;
        } else if self.bits_per_entry <= 8 {
            if let Some(palette) = &self.palette {
                VarIntWrapper(palette.len() as i32).encode(w)?;
                for &entry in palette {
                    VarIntWrapper(entry).encode(w)?;
                }
            } else {
                VarIntWrapper(0).encode(w)?;
            }
        }

        self.data.iter().try_for_each(|long| long.encode(w))?;
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let bits_per_entry = u8::decode(r)?;

        let palette = if bits_per_entry == 0 {
            let single = VarIntWrapper::decode(r)?.0;
            Some(vec![single])
        } else if bits_per_entry <= 8 {
            let len = VarIntWrapper::decode(r)?.0 as usize;
            Some(ChunkDataCodec::decode_vec(len, || {
                VarIntWrapper::decode(r).map(|value| value.0)
            })?)
        } else {
            None
        };

        let entry_count = if bits_per_entry == 0 { 0 } else { 4096 };
        let data_len = ChunkDataCodec::storage_len(bits_per_entry, entry_count);
        let data = ChunkDataCodec::decode_vec(data_len, || u64::decode(r))?;

        Ok(PalettedContainer {
            bits_per_entry,
            palette,
            data,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub block_count: i16,
    pub block_states: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl DataType for ChunkSection {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.block_count.encode(w)?;
        self.block_states.encode(w)?;
        self.biomes.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(ChunkSection {
            block_count: i16::decode(r)?,
            block_states: PalettedContainer::decode(r)?,
            biomes: PalettedContainer::decode(r)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HeightmapEntry {
    pub kind: i32,
    pub data: Vec<i64>,
}

impl DataType for HeightmapEntry {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.kind).encode(w)?;
        VarIntWrapper(self.data.len() as i32).encode(w)?;
        for &value in &self.data {
            value.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let kind = VarIntWrapper::decode(r)?.0;
        let len = VarIntWrapper::decode(r)?.0 as usize;
        let data = ChunkDataCodec::decode_vec(len, || i64::decode(r))?;

        Ok(Self { kind, data })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub block_entity_type: BlockEntityType,
    pub nbt: NbtCompound,
}

impl BlockEntity {
    pub fn new(
        x: i32,
        y: i32,
        z: i32,
        block_entity_type: BlockEntityType,
        nbt: NbtCompound,
    ) -> Self {
        Self {
            packed_xz: (((x & 15) << 4) | (z & 15)) as u8,
            y: y as i16,
            block_entity_type,
            nbt,
        }
    }
}

impl DataType for BlockEntity {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.packed_xz.encode(w)?;
        self.y.encode(w)?;
        VarIntWrapper(self.block_entity_type.id()).encode(w)?;
        self.nbt.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let packed_xz = u8::decode(r)?;
        let y = i16::decode(r)?;
        let block_entity_type_id = VarIntWrapper::decode(r)?.0;
        let block_entity_type =
            BlockEntityType::from_id(block_entity_type_id).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Unknown block entity type")
            })?;
        let nbt = NbtCompound::decode(r)?;
        Ok(Self {
            packed_xz,
            y,
            block_entity_type,
            nbt,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: Vec<HeightmapEntry>,
    pub sections: Vec<ChunkSection>,
    pub block_entities: Vec<BlockEntity>,
}

impl DataType for ChunkData {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.heightmaps.len() as i32).encode(w)?;
        for heightmap in &self.heightmaps {
            heightmap.encode(w)?;
        }

        let mut section_buf = Vec::new();
        for section in &self.sections {
            section.encode(&mut section_buf)?;
        }
        VarIntWrapper(section_buf.len() as i32).encode(w)?;
        w.write_all(&section_buf)?;

        VarIntWrapper(self.block_entities.len() as i32).encode(w)?;
        for block_entity in &self.block_entities {
            block_entity.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let heightmap_count = VarIntWrapper::decode(r)?.0 as usize;
        let heightmaps = ChunkDataCodec::decode_vec(heightmap_count, || HeightmapEntry::decode(r))?;

        let len = VarIntWrapper::decode(r)?.0 as usize;
        let mut section_data = vec![0u8; len];
        r.read_exact(&mut section_data)?;

        let entity_count = VarIntWrapper::decode(r)?.0 as usize;
        let sections = ChunkDataCodec::decode_sections(section_data)?;
        let block_entities = ChunkDataCodec::decode_block_entities(r, entity_count)?;

        Ok(ChunkData {
            heightmaps,
            sections,
            block_entities,
        })
    }
}

struct ChunkDataCodec;

impl ChunkDataCodec {
    fn decode_block_entities<R: Read>(
        r: &mut R,
        entity_count: usize,
    ) -> io::Result<Vec<BlockEntity>> {
        ChunkDataCodec::decode_vec(entity_count, || BlockEntity::decode(r))
    }

    fn decode_sections(section_data: Vec<u8>) -> io::Result<Vec<ChunkSection>> {
        let mut section_cursor = std::io::Cursor::new(section_data);
        std::iter::from_fn(|| {
            if section_cursor.position() >= section_cursor.get_ref().len() as u64 {
                return None;
            }

            Some(ChunkSection::decode(&mut section_cursor))
        })
        .collect()
    }

    fn decode_vec<T>(
        len: usize,
        mut decode_item: impl FnMut() -> io::Result<T>,
    ) -> io::Result<Vec<T>> {
        (0..len).map(|_| decode_item()).collect()
    }

    fn storage_len(bits_per_entry: u8, entry_count: usize) -> usize {
        if bits_per_entry == 0 {
            return 0;
        }
        let entries_per_long = 64 / bits_per_entry as usize;
        entry_count.div_ceil(entries_per_long)
    }
}
