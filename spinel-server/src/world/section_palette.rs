use crate::world::Block;
pub(crate) use crate::world::section_palette_encoding::SectionPaletteError;
use crate::world::section_palette_encoding::{network_palette, single_network_palette};
use spinel_network::types::chunk::PalettedContainer;

pub(crate) trait PaletteEntry {
    fn palette_id(&self) -> i32;
}

impl PaletteEntry for Block {
    fn palette_id(&self) -> i32 {
        self.state_id()
    }
}

#[derive(Clone)]
pub(crate) enum SectionPalette<T, const ENTRY_COUNT: usize> {
    Single(T),
    Indirect(Vec<T>),
}

impl<T, const ENTRY_COUNT: usize> SectionPalette<T, ENTRY_COUNT>
where
    T: Clone + Eq,
{
    pub(crate) fn new(default_entry: T) -> Self {
        Self::Single(default_entry)
    }

    pub(crate) fn get(&self, entry_index: usize) -> Option<T> {
        if entry_index >= ENTRY_COUNT {
            return None;
        }
        match self {
            Self::Single(entry) => Some(entry.clone()),
            Self::Indirect(entries) => entries.get(entry_index).cloned(),
        }
    }

    pub(crate) fn set(&mut self, entry_index: usize, entry: T) -> bool {
        if entry_index >= ENTRY_COUNT {
            return false;
        }
        match self {
            Self::Single(current_entry) if *current_entry == entry => true,
            Self::Single(current_entry) => {
                let current_entry = current_entry.clone();
                let mut entries = vec![current_entry; ENTRY_COUNT];
                entries[entry_index] = entry;
                *self = Self::Indirect(entries);
                self.compress_if_single();
                true
            }
            Self::Indirect(entries) => {
                entries[entry_index] = entry;
                self.compress_if_single();
                true
            }
        }
    }

    pub(crate) fn fill(&mut self, entry: T) {
        *self = Self::Single(entry);
    }

    pub(crate) fn entries(&self) -> Vec<T> {
        match self {
            Self::Single(entry) => vec![entry.clone(); ENTRY_COUNT],
            Self::Indirect(entries) => entries.clone(),
        }
    }

    pub(crate) fn try_to_network(
        &self,
        minimum_bits_per_entry: u8,
        indirect_bits_limit: u8,
        palette_id: impl Fn(&T) -> Option<i32>,
    ) -> Result<PalettedContainer, SectionPaletteError> {
        let entries = self.entries();
        network_palette(
            &entries,
            minimum_bits_per_entry,
            indirect_bits_limit,
            palette_id,
        )
    }

    fn compress_if_single(&mut self) {
        let Self::Indirect(entries) = self else {
            return;
        };
        let Some(first_entry) = entries.first().cloned() else {
            return;
        };
        if entries.iter().all(|entry| *entry == first_entry) {
            *self = Self::Single(first_entry);
        }
    }
}

impl<T, const ENTRY_COUNT: usize> SectionPalette<T, ENTRY_COUNT>
where
    T: Clone + Eq + PaletteEntry,
{
    pub(crate) fn to_network(
        &self,
        minimum_bits_per_entry: u8,
        indirect_bits_limit: u8,
    ) -> PalettedContainer {
        self.try_to_network(minimum_bits_per_entry, indirect_bits_limit, |entry| {
            Some(entry.palette_id())
        })
        .unwrap_or_else(|_| single_network_palette(0))
    }
}
