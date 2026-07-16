use flate2::{Compression, read::GzDecoder, read::ZlibDecoder, write::ZlibEncoder};
use spinel_nbt::{Nbt, NbtCompound};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_ENTRY_COUNT: usize = 1024;
const SECTOR_BYTES: usize = 4096;
const REGION_HEADER_BYTES: usize = MAX_ENTRY_COUNT * 8;
const CHUNK_HEADER_BYTES: usize = 5;
const MAX_SECTOR_COUNT: usize = 255;
const COMPRESSION_GZIP: u8 = 1;
const COMPRESSION_ZLIB: u8 = 2;
const COMPRESSION_NONE: u8 = 3;

pub struct RegionFile {
    file: File,
    locations: [u32; MAX_ENTRY_COUNT],
    timestamps: [u32; MAX_ENTRY_COUNT],
    free_sectors: Vec<bool>,
    header_is_dirty: bool,
}

impl RegionFile {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut region_file = Self {
            file,
            locations: [0; MAX_ENTRY_COUNT],
            timestamps: [0; MAX_ENTRY_COUNT],
            free_sectors: Vec::new(),
            header_is_dirty: false,
        };
        region_file.read_header()?;
        Ok(region_file)
    }

    pub fn file_name(region_x: i32, region_z: i32) -> String {
        format!("r.{region_x}.{region_z}.mca")
    }

    pub fn has_chunk_data(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.locations[chunk_region_index(chunk_x, chunk_z)] != 0
    }

    pub fn read_chunk_data(
        &mut self,
        chunk_x: i32,
        chunk_z: i32,
    ) -> io::Result<Option<NbtCompound>> {
        if !self.has_chunk_data(chunk_x, chunk_z) {
            return Ok(None);
        }
        let location = self.locations[chunk_region_index(chunk_x, chunk_z)];
        self.file.seek(SeekFrom::Start(
            u64::from(location >> 8) * SECTOR_BYTES as u64,
        ))?;
        let mut length_bytes = [0; 4];
        self.file.read_exact(&mut length_bytes)?;
        let chunk_length = u32::from_be_bytes(length_bytes) as usize;
        if chunk_length == 0 {
            return Ok(None);
        }
        let mut compression_type = [0; 1];
        self.file.read_exact(&mut compression_type)?;
        let mut encoded_chunk = vec![0; chunk_length.saturating_sub(1)];
        self.file.read_exact(&mut encoded_chunk)?;
        let decoded_chunk = decode_chunk_payload(compression_type[0], &encoded_chunk)?;
        let mut decoded_slice = decoded_chunk.as_slice();
        let (_, nbt) = Nbt::read_from_stream(&mut decoded_slice)?;
        let Nbt::Compound(chunk_data) = nbt else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Anvil chunk root must be a compound.",
            ));
        };
        Ok(Some(chunk_data))
    }

    pub fn write_chunk_data(
        &mut self,
        chunk_x: i32,
        chunk_z: i32,
        chunk_data: NbtCompound,
    ) -> io::Result<()> {
        let encoded_chunk = encode_chunk_payload(chunk_data)?;
        let chunk_length = CHUNK_HEADER_BYTES + encoded_chunk.len();
        let sector_count = chunk_length.div_ceil(SECTOR_BYTES);
        if sector_count > MAX_SECTOR_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Chunk data is too large to fit in a region file.",
            ));
        }
        let chunk_index = chunk_region_index(chunk_x, chunk_z);
        let previous_location = self.locations[chunk_index];
        let first_sector = self
            .find_free_sectors(sector_count)
            .unwrap_or_else(|| self.allocate_sectors(sector_count));
        let new_location = ((first_sector as u32) << 8) | sector_count as u32;
        self.mark_location(previous_location, true);
        self.mark_location(new_location, false);
        self.file
            .seek(SeekFrom::Start(first_sector as u64 * SECTOR_BYTES as u64))?;
        self.file.write_all(&(chunk_length as u32).to_be_bytes())?;
        self.file.write_all(&[COMPRESSION_ZLIB])?;
        self.file.write_all(&encoded_chunk)?;
        let padding_bytes = sector_count * SECTOR_BYTES - chunk_length;
        if padding_bytes > 0 {
            self.file.write_all(&vec![0; padding_bytes])?;
        }
        self.locations[chunk_index] = new_location;
        self.timestamps[chunk_index] = current_timestamp_seconds();
        self.header_is_dirty = true;
        self.write_header()
    }

    fn read_header(&mut self) -> io::Result<()> {
        if self.file.metadata()?.len() < REGION_HEADER_BYTES as u64 {
            self.file.seek(SeekFrom::Start(0))?;
            self.file.write_all(&vec![0; REGION_HEADER_BYTES])?;
        }
        let file_length = self.file.metadata()?.len();
        let total_sectors = file_length.div_ceil(SECTOR_BYTES as u64).max(2) as usize;
        self.free_sectors = vec![true; total_sectors];
        self.free_sectors[0] = false;
        self.free_sectors[1] = false;
        self.file.seek(SeekFrom::Start(0))?;
        let mut header_bytes = vec![0; REGION_HEADER_BYTES];
        self.file.read_exact(&mut header_bytes)?;
        for entry_index in 0..MAX_ENTRY_COUNT {
            let location_offset = entry_index * 4;
            let location = u32::from_be_bytes([
                header_bytes[location_offset],
                header_bytes[location_offset + 1],
                header_bytes[location_offset + 2],
                header_bytes[location_offset + 3],
            ]);
            self.locations[entry_index] = location;
            if location != 0 {
                self.mark_location_in_free_sectors(location, false);
            }
            let timestamp_offset = MAX_ENTRY_COUNT * 4 + entry_index * 4;
            self.timestamps[entry_index] = u32::from_be_bytes([
                header_bytes[timestamp_offset],
                header_bytes[timestamp_offset + 1],
                header_bytes[timestamp_offset + 2],
                header_bytes[timestamp_offset + 3],
            ]);
        }
        self.header_is_dirty = false;
        Ok(())
    }

    fn write_header(&mut self) -> io::Result<()> {
        if !self.header_is_dirty {
            return Ok(());
        }
        let mut header_bytes = vec![0; REGION_HEADER_BYTES];
        for entry_index in 0..MAX_ENTRY_COUNT {
            let location_offset = entry_index * 4;
            header_bytes[location_offset..location_offset + 4]
                .copy_from_slice(&self.locations[entry_index].to_be_bytes());
            let timestamp_offset = MAX_ENTRY_COUNT * 4 + entry_index * 4;
            header_bytes[timestamp_offset..timestamp_offset + 4]
                .copy_from_slice(&self.timestamps[entry_index].to_be_bytes());
        }
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&header_bytes)?;
        self.header_is_dirty = false;
        Ok(())
    }

    fn find_free_sectors(&self, sector_count: usize) -> Option<usize> {
        self.free_sectors
            .windows(sector_count)
            .position(|sectors| sectors.iter().all(|sector_is_free| *sector_is_free))
    }

    fn allocate_sectors(&mut self, sector_count: usize) -> usize {
        let first_sector = self.free_sectors.len();
        self.free_sectors
            .extend(std::iter::repeat_n(true, sector_count));
        first_sector
    }

    fn mark_location(&mut self, location: u32, sector_is_free: bool) {
        self.mark_location_in_free_sectors(location, sector_is_free);
        self.header_is_dirty = true;
    }

    fn mark_location_in_free_sectors(&mut self, location: u32, sector_is_free: bool) {
        let sector_count = (location & 0xff) as usize;
        let sector_start = (location >> 8) as usize;
        if sector_count == 0 {
            return;
        }
        let required_length = sector_start + sector_count;
        if required_length > self.free_sectors.len() {
            self.free_sectors.resize(required_length, true);
        }
        self.free_sectors[sector_start..required_length]
            .iter_mut()
            .for_each(|sector| *sector = sector_is_free);
    }
}

fn chunk_region_index(chunk_x: i32, chunk_z: i32) -> usize {
    ((chunk_z.rem_euclid(32) as usize) << 5) | chunk_x.rem_euclid(32) as usize
}

fn decode_chunk_payload(compression_type: u8, encoded_chunk: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoded_chunk = Vec::new();
    match compression_type {
        COMPRESSION_GZIP => {
            GzDecoder::new(encoded_chunk).read_to_end(&mut decoded_chunk)?;
        }
        COMPRESSION_ZLIB => {
            ZlibDecoder::new(encoded_chunk).read_to_end(&mut decoded_chunk)?;
        }
        COMPRESSION_NONE => decoded_chunk.extend_from_slice(encoded_chunk),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported Anvil chunk compression type.",
            ));
        }
    };
    Ok(decoded_chunk)
}

fn encode_chunk_payload(chunk_data: NbtCompound) -> io::Result<Vec<u8>> {
    let mut nbt_payload = Vec::new();
    Nbt::Compound(chunk_data).write("", &mut nbt_payload)?;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&nbt_payload)?;
    encoder.finish()
}

fn current_timestamp_seconds() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .min(u32::MAX as u64) as u32
}
