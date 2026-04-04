use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use spinel_nbt::NbtCompound;
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
            let val = self
                .palette
                .as_ref()
                .and_then(|p| p.first())
                .copied()
                .unwrap_or(0);
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

        VarIntWrapper(self.data.len() as i32).encode(w)?;
        for &long in &self.data {
            long.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let bits_per_entry = u8::decode(r)?;

        let palette = if bits_per_entry == 0 {
            let single = VarIntWrapper::decode(r)?.0;
            Some(vec![single])
        } else if bits_per_entry <= 8 {
            let len = VarIntWrapper::decode(r)?.0 as usize;
            let mut pal = Vec::with_capacity(len);
            for _ in 0..len {
                pal.push(VarIntWrapper::decode(r)?.0);
            }
            Some(pal)
        } else {
            None
        };

        let data_len = VarIntWrapper::decode(r)?.0 as usize;
        let mut data = Vec::with_capacity(data_len);
        for _ in 0..data_len {
            data.push(u64::decode(r)?);
        }

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

impl ChunkSection {
    pub fn empty() -> Self {
        Self {
            block_count: 0,
            block_states: PalettedContainer {
                bits_per_entry: 0,
                palette: Some(vec![0]),
                data: vec![],
            },
            biomes: PalettedContainer {
                bits_per_entry: 0,
                palette: Some(vec![1]),
                data: vec![],
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub heightmaps: NbtCompound,
    pub sections: Vec<ChunkSection>,
    pub block_entities: Vec<NbtCompound>,
}

impl DataType for ChunkData {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.heightmaps.encode(w)?;

        let mut section_buf = Vec::new();
        for section in &self.sections {
            section.encode(&mut section_buf)?;
        }
        VarIntWrapper(section_buf.len() as i32).encode(w)?;
        w.write_all(&section_buf)?;

        VarIntWrapper(self.block_entities.len() as i32).encode(w)?;
        for entity in &self.block_entities {
            entity.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let heightmaps = NbtCompound::decode(r)?;

        let len = VarIntWrapper::decode(r)?.0 as usize;
        let mut section_data = vec![0u8; len];
        r.read_exact(&mut section_data)?;

        let mut sections = Vec::new();
        let mut section_cursor = std::io::Cursor::new(section_data);
        while section_cursor.position() < section_cursor.get_ref().len() as u64 {
            sections.push(ChunkSection::decode(&mut section_cursor)?);
        }

        let entity_count = VarIntWrapper::decode(r)?.0 as usize;
        let mut block_entities = Vec::with_capacity(entity_count);
        for _ in 0..entity_count {
            block_entities.push(NbtCompound::decode(r)?);
        }

        Ok(ChunkData {
            heightmaps,
            sections,
            block_entities,
        })
    }
}
