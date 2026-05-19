use spinel_network::types::chunk::PalettedContainer;

pub(crate) fn network_palette<T>(
    entries: &[T],
    minimum_bits_per_entry: u8,
    indirect_bits_limit: u8,
    palette_id: impl Fn(&T) -> Option<i32>,
) -> Result<PalettedContainer, SectionPaletteError> {
    let palette_ids = palette_ids(entries, &palette_id)?;
    if palette_ids.len() == 1 {
        return Ok(single_network_palette(palette_ids[0]));
    }
    let bits_per_entry = bits_per_entry(palette_ids.len(), minimum_bits_per_entry);
    if bits_per_entry > indirect_bits_limit {
        return direct_network_palette(entries, bits_per_entry, palette_id);
    }
    Ok(PalettedContainer {
        bits_per_entry,
        palette: Some(palette_ids.clone()),
        data: pack_palette_indices(entries, &palette_ids, bits_per_entry, palette_id)?,
    })
}

fn direct_network_palette<T>(
    entries: &[T],
    bits_per_entry: u8,
    palette_id: impl Fn(&T) -> Option<i32>,
) -> Result<PalettedContainer, SectionPaletteError> {
    Ok(PalettedContainer {
        bits_per_entry,
        palette: None,
        data: pack_direct_entries(entries, bits_per_entry, palette_id)?,
    })
}

pub(crate) fn single_network_palette(palette_id: i32) -> PalettedContainer {
    PalettedContainer {
        bits_per_entry: 0,
        palette: Some(vec![palette_id]),
        data: Vec::new(),
    }
}

fn palette_ids<T>(
    entries: &[T],
    palette_id: &impl Fn(&T) -> Option<i32>,
) -> Result<Vec<i32>, SectionPaletteError> {
    entries.iter().try_fold(Vec::new(), |mut ids, entry| {
        let id = palette_id(entry).ok_or(SectionPaletteError::MissingEntry)?;
        if !ids.contains(&id) {
            ids.push(id);
        }
        Ok(ids)
    })
}

fn pack_palette_indices<T>(
    entries: &[T],
    palette_ids: &[i32],
    bits_per_entry: u8,
    palette_id: impl Fn(&T) -> Option<i32>,
) -> Result<Vec<u64>, SectionPaletteError> {
    pack_entries(entries, bits_per_entry, |entry| {
        let id = palette_id(entry)?;
        palette_ids
            .iter()
            .position(|palette_id| *palette_id == id)
            .map(|index| index as u64)
    })
}

fn pack_direct_entries<T>(
    entries: &[T],
    bits_per_entry: u8,
    palette_id: impl Fn(&T) -> Option<i32>,
) -> Result<Vec<u64>, SectionPaletteError> {
    pack_entries(entries, bits_per_entry, |entry| {
        palette_id(entry).map(|id| id as u64)
    })
}

fn pack_entries<T>(
    entries: &[T],
    bits_per_entry: u8,
    entry_value: impl Fn(&T) -> Option<u64>,
) -> Result<Vec<u64>, SectionPaletteError> {
    let entries_per_long = 64 / bits_per_entry as usize;
    let mut packed_entries = vec![0; entries.len().div_ceil(entries_per_long)];
    entries
        .iter()
        .enumerate()
        .try_for_each(|(entry_index, entry)| {
            let value = entry_value(entry).ok_or(SectionPaletteError::MissingEntry)?;
            let packed_index = entry_index / entries_per_long;
            let bit_offset = (entry_index % entries_per_long) * bits_per_entry as usize;
            packed_entries[packed_index] |= value << bit_offset;
            Ok(())
        })?;
    Ok(packed_entries)
}

fn bits_per_entry(palette_len: usize, minimum_bits_per_entry: u8) -> u8 {
    let palette_index_limit = palette_len.saturating_sub(1);
    let required_bits = usize::BITS - palette_index_limit.leading_zeros();
    required_bits.max(minimum_bits_per_entry as u32) as u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionPaletteError {
    MissingEntry,
}
