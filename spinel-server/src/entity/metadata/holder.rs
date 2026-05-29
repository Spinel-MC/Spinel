use crate::entity::metadata::{MetadataBitMaskDefinition, MetadataDefinition};
use spinel_network::types::entity_metadata::{MetadataEntry, MetadataValue};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetadataHolder {
    entries: BTreeMap<u8, MetadataValue>,
    dirty_entries: BTreeMap<u8, MetadataValue>,
}

impl MetadataHolder {
    pub fn value(&self, definition: &MetadataDefinition) -> MetadataValue {
        self.entries
            .get(&definition.index())
            .cloned()
            .unwrap_or_else(|| definition.default_value().clone())
    }

    pub fn set(&mut self, definition: &MetadataDefinition, value: MetadataValue) {
        self.entries.insert(definition.index(), value.clone());
        self.dirty_entries.insert(definition.index(), value);
    }

    pub fn flag(&self, definition: &MetadataBitMaskDefinition) -> bool {
        let Some(MetadataValue::Byte(flags)) = self.entries.get(&definition.index()) else {
            return definition.default_value();
        };
        flags & definition.bit_mask() == definition.bit_mask()
    }

    pub fn set_flag(&mut self, definition: &MetadataBitMaskDefinition, flag_is_enabled: bool) {
        let current_flags = match self.entries.get(&definition.index()) {
            Some(MetadataValue::Byte(flags)) => *flags,
            _ => 0,
        };
        let updated_flags = if flag_is_enabled {
            current_flags | definition.bit_mask()
        } else {
            current_flags & !definition.bit_mask()
        };
        let updated_value = MetadataValue::Byte(updated_flags);
        self.entries
            .insert(definition.index(), updated_value.clone());
        self.dirty_entries.insert(definition.index(), updated_value);
    }

    pub fn entries(&self) -> Vec<MetadataEntry> {
        self.entries
            .iter()
            .map(|(index, value)| MetadataEntry {
                index: *index,
                value: value.clone(),
            })
            .collect()
    }

    pub fn drain_dirty_entries(&mut self) -> Vec<MetadataEntry> {
        std::mem::take(&mut self.dirty_entries)
            .into_iter()
            .map(|(index, value)| MetadataEntry { index, value })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::MetadataHolder;
    use crate::entity::metadata::definitions;
    use spinel_network::types::entity_metadata::MetadataValue;

    #[test]
    fn metadata_holder_returns_minestom_default_when_entry_is_unset() {
        let metadata = MetadataHolder::default();

        assert_eq!(
            metadata.value(&definitions::air_ticks()),
            MetadataValue::VarInt(300)
        );
    }

    #[test]
    fn metadata_holder_tracks_dirty_entries_until_drain() {
        let mut metadata = MetadataHolder::default();

        metadata.set(&definitions::air_ticks(), MetadataValue::VarInt(10));
        let dirty_entries = metadata.drain_dirty_entries();

        assert_eq!(dirty_entries.len(), 1);
        assert_eq!(dirty_entries[0].index, 1);
        assert!(metadata.drain_dirty_entries().is_empty());
    }

    #[test]
    fn metadata_holder_bitmask_entries_share_the_base_byte() {
        let mut metadata = MetadataHolder::default();

        metadata.set_flag(&definitions::is_on_fire(), true);
        metadata.set_flag(&definitions::is_crouching(), true);

        assert!(metadata.flag(&definitions::is_on_fire()));
        assert!(metadata.flag(&definitions::is_crouching()));
        assert_eq!(
            metadata.value(&definitions::entity_flags()),
            MetadataValue::Byte(0x03)
        );
    }
}
