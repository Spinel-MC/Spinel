use crate::world::anvil::region_file::RegionFile;
use crate::world::{
    Biome, BlockEntity, BlockInstance, BlockLookupCondition, BlockPosition, Chunk, ChunkLoader,
    ChunkPosition, ChunkSection,
};
use spinel_nbt::{Nbt, NbtCompound, Taggable};
use spinel_registry::{Identifier, RegistryKey};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

const CHUNK_SECTION_BLOCK_COUNT: usize = 4096;
const CHUNK_SECTION_BIOME_COUNT: usize = 64;
const DATA_VERSION: i32 = 3953;

pub struct AnvilChunkLoader {
    world_directory: PathBuf,
    region_directory: PathBuf,
    file_creation_lock: Mutex<()>,
    loaded_region_files: Mutex<HashMap<String, Arc<Mutex<RegionFile>>>>,
    loaded_chunks_by_region: Mutex<HashMap<(i32, i32), HashSet<(i32, i32)>>>,
}

impl AnvilChunkLoader {
    pub fn new(world_directory: PathBuf) -> io::Result<Self> {
        let region_directory = world_directory.join("region");
        fs::create_dir_all(&region_directory)?;
        Ok(Self {
            world_directory,
            region_directory,
            file_creation_lock: Mutex::new(()),
            loaded_region_files: Mutex::new(HashMap::new()),
            loaded_chunks_by_region: Mutex::new(HashMap::new()),
        })
    }

    pub fn from_path(path: impl Into<PathBuf>) -> io::Result<Self> {
        Self::new(path.into())
    }

    fn get_region_file(
        &self,
        position: ChunkPosition,
        creation: RegionFileCreation,
    ) -> io::Result<Option<Arc<Mutex<RegionFile>>>> {
        let region_position = region_position(position);
        let region_file_name = RegionFile::file_name(region_position.0, region_position.1);
        if let Some(region_file) = lock_mutex(&self.loaded_region_files)?
            .get(&region_file_name)
            .cloned()
        {
            return Ok(Some(region_file));
        }
        let creation_guard = lock_mutex(&self.file_creation_lock)?;
        let mut loaded_region_files = lock_mutex(&self.loaded_region_files)?;
        if let Some(region_file) = loaded_region_files.get(&region_file_name).cloned() {
            drop(creation_guard);
            return Ok(Some(region_file));
        }
        let region_file_path = self.region_directory.join(&region_file_name);
        if creation == RegionFileCreation::Existing && !region_file_path.exists() {
            drop(creation_guard);
            return Ok(None);
        }
        if let Some(region_file_parent) = region_file_path.parent() {
            fs::create_dir_all(region_file_parent)?;
        }
        let region_file = Arc::new(Mutex::new(RegionFile::open(&region_file_path)?));
        loaded_region_files.insert(region_file_name, Arc::clone(&region_file));
        lock_mutex(&self.loaded_chunks_by_region)?
            .entry(region_position)
            .or_default();
        drop(creation_guard);
        Ok(Some(region_file))
    }

    fn record_loaded_chunk(&self, position: ChunkPosition) -> io::Result<()> {
        lock_mutex(&self.loaded_chunks_by_region)?
            .entry(region_position(position))
            .or_default()
            .insert((position.x, position.z));
        Ok(())
    }

    fn unload_region_if_empty(&self, position: ChunkPosition) -> io::Result<()> {
        let region_position = region_position(position);
        let mut loaded_chunks_by_region = lock_mutex(&self.loaded_chunks_by_region)?;
        let Some(loaded_chunks) = loaded_chunks_by_region.get_mut(&region_position) else {
            return Ok(());
        };
        loaded_chunks.remove(&(position.x, position.z));
        if !loaded_chunks.is_empty() {
            return Ok(());
        }
        loaded_chunks_by_region.remove(&region_position);
        let region_file_name = RegionFile::file_name(region_position.0, region_position.1);
        lock_mutex(&self.loaded_region_files)?.remove(&region_file_name);
        Ok(())
    }
}

impl ChunkLoader for AnvilChunkLoader {
    fn load_chunk(&self, position: ChunkPosition) -> io::Result<Option<Chunk>> {
        if !self.world_directory.exists() {
            return Ok(None);
        }
        let Some(region_file) = self.get_region_file(position, RegionFileCreation::Existing)?
        else {
            return Ok(None);
        };
        let Some(chunk_data) = lock_mutex(&region_file)?.read_chunk_data(position.x, position.z)?
        else {
            return Ok(None);
        };
        let chunk = decode_chunk(position, chunk_data)?;
        self.record_loaded_chunk(position)?;
        Ok(Some(chunk))
    }

    fn save_chunk(&self, chunk: &Chunk) -> io::Result<()> {
        let position = ChunkPosition::from(chunk);
        let region_file = self
            .get_region_file(position, RegionFileCreation::Create)?
            .ok_or_else(|| io::Error::other("Anvil region file was not opened."))?;
        lock_mutex(&region_file)?.write_chunk_data(position.x, position.z, chunk_nbt(chunk))?;
        self.record_loaded_chunk(position)
    }

    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()> {
        let position = ChunkPosition::from(&*chunk);
        chunk.unload();
        self.unload_region_if_empty(position)
    }

    fn supports_parallel_loading(&self) -> bool {
        true
    }

    fn supports_parallel_saving(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RegionFileCreation {
    Existing,
    Create,
}

fn decode_chunk(position: ChunkPosition, mut chunk_data: NbtCompound) -> io::Result<Chunk> {
    let mut chunk = Chunk::new_with_generation(position, false);
    if chunk_has_full_status(&chunk_data) {
        load_sections(&mut chunk, &chunk_data)?;
        load_block_entities(&mut chunk, &chunk_data);
        if let Some(Nbt::Compound(heightmaps)) = chunk_data.get("Heightmaps") {
            chunk.load_heightmaps_from_nbt(heightmaps);
        }
    }
    chunk_data.remove("Heightmaps");
    chunk_data.remove("sections");
    chunk_data.remove("block_entities");
    chunk.tag_handler_mut().update_content(chunk_data);
    chunk.mark_loaded_from_storage();
    Ok(chunk)
}

fn chunk_has_full_status(chunk_data: &NbtCompound) -> bool {
    match chunk_data.get("status") {
        Some(Nbt::String(status)) => status.is_empty() || status == "minecraft:full",
        Some(_) => false,
        _ => true,
    }
}

fn load_sections(chunk: &mut Chunk, chunk_data: &NbtCompound) -> io::Result<()> {
    let Some(Nbt::List(sections)) = chunk_data.get("sections") else {
        return Ok(());
    };
    for section in sections.iter() {
        let Nbt::Compound(section_compound) = section else {
            continue;
        };
        load_section(chunk, section_compound)?;
    }
    Ok(())
}

fn load_section(chunk: &mut Chunk, section_data: &NbtCompound) -> io::Result<()> {
    let Some(section_y) = section_y(section_data) else {
        return Ok(());
    };
    if section_y < chunk.min_section() || section_y >= chunk.max_section() {
        return Ok(());
    }
    load_section_light(chunk, section_y, section_data)?;
    load_section_biomes(chunk, section_y, section_data);
    load_section_blocks(chunk, section_y, section_data)
}

fn load_section_light(
    chunk: &mut Chunk,
    section_y: i32,
    section_data: &NbtCompound,
) -> io::Result<()> {
    let Some(section) = chunk.section_mut(section_y) else {
        return Ok(());
    };
    if let Some(Nbt::ByteArray(sky_light)) = section_data.get("SkyLight") {
        section
            .set_sky_light(sky_light)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    }
    if let Some(Nbt::ByteArray(block_light)) = section_data.get("BlockLight") {
        section
            .set_block_light(block_light)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    }
    Ok(())
}

fn load_section_biomes(chunk: &mut Chunk, section_y: i32, section_data: &NbtCompound) {
    let Some(Nbt::Compound(biomes)) = section_data.get("biomes") else {
        return;
    };
    let biome_palette = read_biome_palette(biomes);
    if biome_palette.is_empty() {
        return;
    }
    let biome_indices =
        read_palette_indices::<CHUNK_SECTION_BIOME_COUNT>(biomes, biome_palette.len(), 1);
    for (biome_index, palette_index) in biome_indices.into_iter().enumerate() {
        let Some(biome) = biome_palette.get(palette_index).cloned() else {
            continue;
        };
        let x = (biome_index & 3) as i32;
        let z = ((biome_index >> 2) & 3) as i32;
        let y = ((biome_index >> 4) & 3) as i32;
        chunk.set_biome(
            BlockPosition::new(x << 2, (section_y << 4) + (y << 2), z << 2),
            biome,
        );
    }
}

fn load_section_blocks(
    chunk: &mut Chunk,
    section_y: i32,
    section_data: &NbtCompound,
) -> io::Result<()> {
    let Some(Nbt::Compound(block_states)) = section_data.get("block_states") else {
        return Ok(());
    };
    let block_palette = read_block_palette(block_states)?;
    if block_palette.is_empty() {
        return Ok(());
    }
    let block_indices =
        read_palette_indices::<CHUNK_SECTION_BLOCK_COUNT>(block_states, block_palette.len(), 4);
    for (block_index, palette_index) in block_indices.into_iter().enumerate() {
        let Some(block_state) = block_palette.get(palette_index).copied() else {
            continue;
        };
        let x = (block_index & 15) as i32;
        let z = ((block_index >> 4) & 15) as i32;
        let y = ((block_index >> 8) & 15) as i32 + (section_y << 4);
        chunk.set_block_state(BlockPosition::new(x, y, z), block_state);
    }
    Ok(())
}
fn load_block_entities(chunk: &mut Chunk, chunk_data: &NbtCompound) {
    let Some(Nbt::List(block_entities)) = chunk_data.get("block_entities") else {
        return;
    };
    for block_entity in block_entities.iter() {
        let Nbt::Compound(block_entity_data) = block_entity else {
            continue;
        };
        load_block_entity(chunk, block_entity_data);
    }
}

fn load_block_entity(chunk: &mut Chunk, block_entity_data: &NbtCompound) -> Option<()> {
    let position = block_entity_position(block_entity_data)?;
    let block_instance =
        chunk.block_instance_with_condition(position, BlockLookupCondition::None)?;
    let trimmed_nbt = trimmed_block_entity_nbt(block_entity_data);
    if trimmed_nbt.is_empty() {
        return Some(());
    }
    chunk.set_block_instance(position, block_instance.with_nbt(Some(trimmed_nbt)));
    Some(())
}

fn chunk_nbt(chunk: &Chunk) -> NbtCompound {
    let mut chunk_data = chunk.tag_handler().as_compound();
    chunk_data.insert("DataVersion".to_string(), DATA_VERSION);
    chunk_data.insert("xPos".to_string(), chunk.x());
    chunk_data.insert("zPos".to_string(), chunk.z());
    chunk_data.insert("yPos".to_string(), chunk.min_section());
    chunk_data.insert("status".to_string(), "minecraft:full");
    chunk_data.insert("isLightOn".to_string(), true);
    chunk_data.insert("LastUpdate".to_string(), 0i64);
    chunk_data.insert("InhabitedTime".to_string(), 0i64);
    chunk_data.insert("sections".to_string(), section_list(chunk));
    chunk_data.insert("Heightmaps".to_string(), heightmaps_nbt(chunk));
    chunk_data.insert("block_entities".to_string(), block_entities_nbt(chunk));
    chunk_data
}

fn section_list(chunk: &Chunk) -> Nbt {
    Nbt::List(
        chunk
            .sections()
            .iter()
            .map(section_nbt)
            .map(Nbt::Compound)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

fn section_nbt(section: &ChunkSection) -> NbtCompound {
    let mut section_data = NbtCompound::new();
    section_data.insert("Y".to_string(), Nbt::Byte(section.y as i8));
    let sky_light = section.sky_light().to_vec().into_boxed_slice();
    if sky_light.iter().any(|light_level| *light_level != 0) {
        section_data.insert("SkyLight".to_string(), Nbt::ByteArray(sky_light));
    }
    let block_light = section.block_light().to_vec().into_boxed_slice();
    if block_light.iter().any(|light_level| *light_level != 0) {
        section_data.insert("BlockLight".to_string(), Nbt::ByteArray(block_light));
    }
    section_data.insert(
        "block_states".to_string(),
        block_states_nbt(section.block_palette().entries()),
    );
    section_data.insert(
        "biomes".to_string(),
        biomes_nbt(section.biome_palette().entries()),
    );
    section_data
}

fn block_states_nbt(block_states: Vec<crate::world::BlockState>) -> NbtCompound {
    let mut palette_indices = BTreeMap::new();
    let mut palette = Vec::new();
    let block_indices = block_states
        .iter()
        .map(|block_state| {
            let block_instance = BlockInstance::from(*block_state);
            let state = block_instance.state();
            if let Some(index) = palette_indices.get(&state) {
                return *index;
            }
            let index = palette.len();
            palette_indices.insert(state.clone(), index);
            palette.push(block_state_nbt(&block_instance));
            index
        })
        .collect::<Vec<_>>();
    let mut block_states = NbtCompound::new();
    block_states.insert(
        "palette".to_string(),
        Nbt::List(palette.into_iter().map(Nbt::Compound).collect()),
    );
    if palette_indices.len() > 1 {
        block_states.insert(
            "data".to_string(),
            Nbt::LongArray(pack_indices(&block_indices, 4)),
        );
    }
    block_states
}

fn block_state_nbt(block_instance: &BlockInstance) -> NbtCompound {
    let mut block_state = NbtCompound::new();
    block_state.insert(
        "Name".to_string(),
        format!("minecraft:{}", block_instance.block().path()),
    );
    let default_state = block_instance.default_state();
    let non_default_properties = block_instance
        .properties()
        .iter()
        .filter(|property| default_state.property(property.name) != Some(property.value))
        .collect::<Vec<_>>();
    if !non_default_properties.is_empty() {
        let mut properties = NbtCompound::new();
        non_default_properties.iter().for_each(|property| {
            properties.insert(property.name.to_string(), property.value.to_string());
        });
        block_state.insert("Properties".to_string(), properties);
    }
    block_state
}

fn biomes_nbt(biomes: Vec<RegistryKey<Biome>>) -> NbtCompound {
    let mut palette_indices = BTreeMap::new();
    let mut palette = Vec::new();
    let biome_indices = biomes
        .iter()
        .map(|biome| {
            let biome_name = biome.key().to_string();
            if let Some(index) = palette_indices.get(&biome_name) {
                return *index;
            }
            let index = palette.len();
            palette_indices.insert(biome_name.clone(), index);
            palette.push(Nbt::String(biome_name));
            index
        })
        .collect::<Vec<_>>();
    let mut biome_data = NbtCompound::new();
    biome_data.insert("palette".to_string(), Nbt::List(palette.into_boxed_slice()));
    if palette_indices.len() > 1 {
        biome_data.insert(
            "data".to_string(),
            Nbt::LongArray(pack_indices(&biome_indices, 1)),
        );
    }
    biome_data
}

fn heightmaps_nbt(chunk: &Chunk) -> NbtCompound {
    let mut heightmaps = NbtCompound::new();
    heightmaps.insert(
        "MOTION_BLOCKING".to_string(),
        Nbt::LongArray(chunk.motion_blocking_heightmap().into_boxed_slice()),
    );
    heightmaps.insert(
        "WORLD_SURFACE".to_string(),
        Nbt::LongArray(chunk.world_surface_heightmap().into_boxed_slice()),
    );
    heightmaps
}

fn block_entities_nbt(chunk: &Chunk) -> Nbt {
    Nbt::List(
        chunk
            .block_entities()
            .iter()
            .map(block_entity_nbt)
            .map(Nbt::Compound)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

fn block_entity_nbt(block_entity: &BlockEntity) -> NbtCompound {
    let mut block_entity_data = block_entity.nbt().clone();
    let position = block_entity.position();
    block_entity_data.insert(
        "id".to_string(),
        format!("minecraft:{}", block_entity.block_entity_type().key()),
    );
    block_entity_data.insert("x".to_string(), position.x);
    block_entity_data.insert("y".to_string(), position.y);
    block_entity_data.insert("z".to_string(), position.z);
    block_entity_data.insert("keepPacked".to_string(), Nbt::Byte(0));
    block_entity_data
}

fn section_y(section_data: &NbtCompound) -> Option<i32> {
    match section_data.get("Y")? {
        Nbt::Byte(value) => Some(i32::from(*value)),
        Nbt::Int(value) => Some(*value),
        _ => None,
    }
}

fn read_block_palette(block_states: &NbtCompound) -> io::Result<Vec<crate::world::BlockState>> {
    let Some(Nbt::List(palette)) = block_states.get("palette") else {
        return Ok(Vec::new());
    };
    palette
        .iter()
        .filter_map(|entry| {
            let Nbt::Compound(compound) = entry else {
                return None;
            };
            Some(read_block_state(compound))
        })
        .collect()
}

fn read_block_state(compound: &NbtCompound) -> io::Result<crate::world::BlockState> {
    let Some(Nbt::String(name)) = compound.get("Name") else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Anvil block state palette entry is missing a Name.",
        ));
    };
    let state = match compound.get("Properties") {
        Some(Nbt::Compound(properties)) => {
            let properties_text = properties
                .0
                .iter()
                .filter_map(|(property_name, property_value)| match property_value {
                    Nbt::String(property_value) => {
                        Some(format!("{property_name}={property_value}"))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(",");
            format!("{name}[{properties_text}]")
        }
        _ => name.clone(),
    };
    BlockInstance::from_state(&state)
        .map(|block_instance| block_instance.block_state())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Unknown block state."))
}

fn read_biome_palette(biomes: &NbtCompound) -> Vec<RegistryKey<Biome>> {
    let Some(Nbt::List(palette)) = biomes.get("palette") else {
        return Vec::new();
    };
    palette
        .iter()
        .filter_map(|entry| {
            let Nbt::String(name) = entry else {
                return None;
            };
            Some(RegistryKey::new(
                Identifier::from_str(name).unwrap_or_else(|_| Identifier::minecraft("plains")),
            ))
        })
        .collect()
}

fn read_palette_indices<const ENTRY_COUNT: usize>(
    palette_data: &NbtCompound,
    palette_len: usize,
    minimum_bits_per_entry: u8,
) -> Vec<usize> {
    let Some(Nbt::LongArray(data)) = palette_data.get("data") else {
        return vec![0; ENTRY_COUNT];
    };
    let bits_per_entry =
        bits_to_represent(palette_len.saturating_sub(1)).max(minimum_bits_per_entry);
    let entries_per_word = 64 / bits_per_entry as usize;
    let mask = (1u64 << bits_per_entry) - 1;
    (0..ENTRY_COUNT)
        .map(|entry_index| {
            let word_index = entry_index / entries_per_word;
            let bit_index = (entry_index % entries_per_word) * bits_per_entry as usize;
            data.get(word_index)
                .map(|word| ((*word as u64 >> bit_index) & mask) as usize)
                .unwrap_or(0)
        })
        .collect()
}

fn block_entity_position(block_entity_data: &NbtCompound) -> Option<BlockPosition> {
    Some(BlockPosition::new(
        nbt_int(block_entity_data.get("x")?)?,
        nbt_int(block_entity_data.get("y")?)?,
        nbt_int(block_entity_data.get("z")?)?,
    ))
}

fn trimmed_block_entity_nbt(block_entity_data: &NbtCompound) -> NbtCompound {
    let mut trimmed = block_entity_data.clone();
    trimmed.remove("id");
    trimmed.remove("keepPacked");
    trimmed.remove("x");
    trimmed.remove("y");
    trimmed.remove("z");
    trimmed
}

fn nbt_int(nbt: &Nbt) -> Option<i32> {
    match nbt {
        Nbt::Byte(value) => Some(i32::from(*value)),
        Nbt::Short(value) => Some(i32::from(*value)),
        Nbt::Int(value) => Some(*value),
        _ => None,
    }
}

fn pack_indices(indices: &[usize], minimum_bits_per_entry: u8) -> Box<[i64]> {
    let bits_per_entry =
        bits_to_represent(indices.iter().copied().max().unwrap_or(0)).max(minimum_bits_per_entry);
    let entries_per_word = 64 / bits_per_entry as usize;
    let mut packed = vec![0i64; indices.len().div_ceil(entries_per_word)];
    indices
        .iter()
        .enumerate()
        .for_each(|(entry_index, palette_index)| {
            let word_index = entry_index / entries_per_word;
            let bit_index = (entry_index % entries_per_word) * bits_per_entry as usize;
            packed[word_index] |= (*palette_index as i64) << bit_index;
        });
    packed.into_boxed_slice()
}

fn bits_to_represent(value: usize) -> u8 {
    (usize::BITS - value.leading_zeros()).max(1) as u8
}

fn region_position(position: ChunkPosition) -> (i32, i32) {
    (position.x.div_euclid(32), position.z.div_euclid(32))
}

fn lock_mutex<T>(mutex: &Mutex<T>) -> io::Result<MutexGuard<'_, T>> {
    mutex
        .lock()
        .map_err(|_| io::Error::other("Anvil loader mutex was poisoned."))
}
