use spinel_nbt::NbtCompound;

#[derive(Debug, Clone)]
pub struct PalettedContainer {
    pub bits_per_entry: u8,
    pub palette: Option<Vec<i32>>,
    pub data: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub block_count: i16,
    pub block_states: PalettedContainer,
    pub biomes: PalettedContainer,
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