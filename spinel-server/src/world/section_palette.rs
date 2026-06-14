use crate::world::BlockState;
pub(crate) use crate::world::section_palette_encoding::SectionPaletteError;
use crate::world::section_palette_encoding::{network_palette, single_network_palette};
use spinel_network::types::chunk::PalettedContainer;

#[derive(Clone)]
pub enum SectionPalette<T, const ENTRY_COUNT: usize, const MINIMUM_BITS_PER_ENTRY: u8> {
    Single(T),
    Indirect {
        palette: Vec<T>,
        packed_indices: Vec<u64>,
        bits_per_entry: u8,
    },
}

impl<T, const ENTRY_COUNT: usize, const MINIMUM_BITS_PER_ENTRY: u8>
    SectionPalette<T, ENTRY_COUNT, MINIMUM_BITS_PER_ENTRY>
where
    T: Clone + Eq,
{
    pub fn new(default_entry: T) -> Self {
        Self::Single(default_entry)
    }

    pub fn get(&self, entry_index: usize) -> Option<T> {
        if entry_index >= ENTRY_COUNT {
            return None;
        }
        match self {
            Self::Single(entry) => Some(entry.clone()),
            Self::Indirect {
                palette,
                packed_indices,
                bits_per_entry,
            } => palette
                .get(read_packed_index(
                    packed_indices,
                    *bits_per_entry,
                    entry_index,
                ))
                .cloned(),
        }
    }

    pub fn set(&mut self, entry_index: usize, entry: T) -> bool {
        if entry_index >= ENTRY_COUNT {
            return false;
        }
        match self {
            Self::Single(current_entry) if *current_entry == entry => true,
            Self::Single(current_entry) => {
                let current_entry = current_entry.clone();
                let bits_per_entry = MINIMUM_BITS_PER_ENTRY.max(1);
                let mut packed_indices =
                    vec![0; packed_index_word_count(ENTRY_COUNT, bits_per_entry)];
                write_packed_index(&mut packed_indices, bits_per_entry, entry_index, 1);
                *self = Self::Indirect {
                    palette: vec![current_entry, entry],
                    packed_indices,
                    bits_per_entry,
                };
                true
            }
            Self::Indirect {
                palette,
                packed_indices,
                bits_per_entry,
            } => {
                let palette_index =
                    palette_index_for_entry(palette, packed_indices, bits_per_entry, entry);
                write_packed_index(packed_indices, *bits_per_entry, entry_index, palette_index);
                true
            }
        }
    }

    pub fn fill(&mut self, entry: T) {
        *self = Self::Single(entry);
    }

    pub(crate) fn fill_range(&mut self, start: usize, end: usize, entry: T) -> bool {
        if start > end || end > ENTRY_COUNT {
            return false;
        }
        if start == 0 && end == ENTRY_COUNT {
            self.fill(entry);
            return true;
        }
        if start == end {
            return true;
        }
        match self {
            Self::Single(current_entry) if *current_entry == entry => true,
            Self::Single(current_entry) => {
                let current_entry = current_entry.clone();
                let bits_per_entry = MINIMUM_BITS_PER_ENTRY.max(1);
                let mut packed_indices =
                    vec![0; packed_index_word_count(ENTRY_COUNT, bits_per_entry)];
                (start..end).for_each(|entry_index| {
                    write_packed_index(&mut packed_indices, bits_per_entry, entry_index, 1);
                });
                *self = Self::Indirect {
                    palette: vec![current_entry, entry],
                    packed_indices,
                    bits_per_entry,
                };
                true
            }
            Self::Indirect {
                palette,
                packed_indices,
                bits_per_entry,
            } => {
                let palette_index =
                    palette_index_for_entry(palette, packed_indices, bits_per_entry, entry);
                (start..end).for_each(|entry_index| {
                    write_packed_index(packed_indices, *bits_per_entry, entry_index, palette_index);
                });
                true
            }
        }
    }

    pub fn entries(&self) -> Vec<T> {
        match self {
            Self::Single(entry) => vec![entry.clone(); ENTRY_COUNT],
            Self::Indirect {
                palette,
                packed_indices,
                bits_per_entry,
            } => (0..ENTRY_COUNT)
                .filter_map(|entry_index| {
                    palette
                        .get(read_packed_index(
                            packed_indices,
                            *bits_per_entry,
                            entry_index,
                        ))
                        .cloned()
                })
                .collect(),
        }
    }

    pub(crate) fn count_matching(&self, predicate: impl Fn(&T) -> bool) -> usize {
        match self {
            Self::Single(entry) => usize::from(predicate(entry)) * ENTRY_COUNT,
            Self::Indirect {
                palette,
                packed_indices,
                bits_per_entry,
            } => (0..ENTRY_COUNT)
                .filter(|entry_index| {
                    palette
                        .get(read_packed_index(
                            packed_indices,
                            *bits_per_entry,
                            *entry_index,
                        ))
                        .is_some_and(&predicate)
                })
                .count(),
        }
    }

    pub(crate) fn try_to_network(
        &self,
        minimum_bits_per_entry: u8,
        indirect_bits_limit: u8,
        palette_id: impl Fn(&T) -> Option<i32>,
    ) -> Result<PalettedContainer, SectionPaletteError> {
        if let Self::Single(entry) = self {
            let Some(entry_id) = palette_id(entry) else {
                return Err(SectionPaletteError::MissingEntry);
            };
            return Ok(single_network_palette(entry_id));
        }
        network_palette(
            &self.entries(),
            minimum_bits_per_entry,
            indirect_bits_limit,
            palette_id,
        )
    }

    #[cfg(test)]
    pub(crate) fn allocated_index_bytes(&self) -> usize {
        match self {
            Self::Single(_) => 0,
            Self::Indirect { packed_indices, .. } => {
                packed_indices.len() * std::mem::size_of::<u64>()
            }
        }
    }
}

impl<const ENTRY_COUNT: usize, const MINIMUM_BITS_PER_ENTRY: u8>
    SectionPalette<BlockState, ENTRY_COUNT, MINIMUM_BITS_PER_ENTRY>
{
    pub(crate) fn to_network(
        &self,
        minimum_bits_per_entry: u8,
        indirect_bits_limit: u8,
    ) -> PalettedContainer {
        self.try_to_network(minimum_bits_per_entry, indirect_bits_limit, |entry| {
            Some(entry.state_id())
        })
        .unwrap_or_else(|_| single_network_palette(0))
    }
}

fn palette_index_for_entry<T: Eq>(
    palette: &mut Vec<T>,
    packed_indices: &mut Vec<u64>,
    bits_per_entry: &mut u8,
    entry: T,
) -> usize {
    if let Some(palette_index) = palette.iter().position(|candidate| candidate == &entry) {
        return palette_index;
    }
    let palette_index = palette.len();
    palette.push(entry);
    let required_bits = bits_to_represent(palette.len() - 1);
    if required_bits > *bits_per_entry {
        *packed_indices = repack_indices(packed_indices, *bits_per_entry, required_bits);
        *bits_per_entry = required_bits;
    }
    palette_index
}

fn bits_to_represent(value: usize) -> u8 {
    (usize::BITS - value.leading_zeros()).max(1) as u8
}

fn packed_index_word_count(entry_count: usize, bits_per_entry: u8) -> usize {
    let entries_per_word = 64 / bits_per_entry as usize;
    entry_count.div_ceil(entries_per_word)
}

fn read_packed_index(packed_indices: &[u64], bits_per_entry: u8, entry_index: usize) -> usize {
    let entries_per_word = 64 / bits_per_entry as usize;
    let word_index = entry_index / entries_per_word;
    let bit_index = (entry_index % entries_per_word) * bits_per_entry as usize;
    let mask = (1u64 << bits_per_entry) - 1;
    ((packed_indices[word_index] >> bit_index) & mask) as usize
}

fn write_packed_index(
    packed_indices: &mut [u64],
    bits_per_entry: u8,
    entry_index: usize,
    palette_index: usize,
) {
    let entries_per_word = 64 / bits_per_entry as usize;
    let word_index = entry_index / entries_per_word;
    let bit_index = (entry_index % entries_per_word) * bits_per_entry as usize;
    let mask = ((1u64 << bits_per_entry) - 1) << bit_index;
    packed_indices[word_index] =
        (packed_indices[word_index] & !mask) | ((palette_index as u64) << bit_index);
}

fn repack_indices(
    packed_indices: &[u64],
    previous_bits_per_entry: u8,
    next_bits_per_entry: u8,
) -> Vec<u64> {
    let previous_entries_per_word = 64 / previous_bits_per_entry as usize;
    let entry_count = packed_indices.len() * previous_entries_per_word;
    let mut repacked_indices = vec![0; packed_index_word_count(entry_count, next_bits_per_entry)];
    (0..entry_count).for_each(|entry_index| {
        let palette_index = read_packed_index(packed_indices, previous_bits_per_entry, entry_index);
        write_packed_index(
            &mut repacked_indices,
            next_bits_per_entry,
            entry_index,
            palette_index,
        );
    });
    repacked_indices
}
