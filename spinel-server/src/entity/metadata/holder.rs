use crate::entity::metadata::{
    MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition,
};
use spinel_network::types::entity_metadata::{MetadataEntry, MetadataValue};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataHolder {
    entries: BTreeMap<u8, MetadataValue>,
    dirty_entries: BTreeMap<u8, MetadataValue>,
    change_notifications_are_enabled: bool,
}

impl Default for MetadataHolder {
    fn default() -> Self {
        Self {
            entries: BTreeMap::new(),
            dirty_entries: BTreeMap::new(),
            change_notifications_are_enabled: true,
        }
    }
}

impl MetadataHolder {
    pub fn get_value(&self, definition: &MetadataDefinition) -> MetadataValue {
        self.entries
            .get(&definition.get_index())
            .cloned()
            .unwrap_or_else(|| definition.get_default_value().clone())
    }

    pub fn set(&mut self, definition: &MetadataDefinition, value: MetadataValue) {
        self.entries.insert(definition.get_index(), value.clone());
        self.dirty_entries.insert(definition.get_index(), value);
    }

    pub fn get_flag(&self, definition: &MetadataBitMaskDefinition) -> bool {
        let Some(MetadataValue::Byte(flags)) = self.entries.get(&definition.get_index()) else {
            return definition.get_default_value();
        };
        flags & definition.get_bit_mask() == definition.get_bit_mask()
    }

    pub fn set_flag(&mut self, definition: &MetadataBitMaskDefinition, flag_is_enabled: bool) {
        let current_flags = match self.entries.get(&definition.get_index()) {
            Some(MetadataValue::Byte(flags)) => *flags,
            _ => 0,
        };
        let updated_flags = if flag_is_enabled {
            current_flags | definition.get_bit_mask()
        } else {
            current_flags & !definition.get_bit_mask()
        };
        let updated_value = MetadataValue::Byte(updated_flags);
        self.entries
            .insert(definition.get_index(), updated_value.clone());
        self.dirty_entries
            .insert(definition.get_index(), updated_value);
    }

    pub fn get_byte(&self, definition: &MetadataByteMaskDefinition) -> i8 {
        let Some(MetadataValue::Byte(current_value)) = self.entries.get(&definition.get_index())
        else {
            return definition.get_default_value();
        };
        (((*current_value as u8) & (definition.get_byte_mask() as u8)) >> definition.get_offset())
            as i8
    }

    pub fn set_byte(&mut self, definition: &MetadataByteMaskDefinition, byte_value: i8) {
        let current_value = match self.entries.get(&definition.get_index()) {
            Some(MetadataValue::Byte(current_value)) => *current_value as u8,
            _ => 0,
        };
        let byte_mask = definition.get_byte_mask() as u8;
        let updated_value = (current_value & !byte_mask)
            | (((byte_value as u8) << definition.get_offset()) & byte_mask);
        let updated_value = MetadataValue::Byte(updated_value as i8);
        self.entries
            .insert(definition.get_index(), updated_value.clone());
        self.dirty_entries
            .insert(definition.get_index(), updated_value);
    }

    pub fn get_entries(&self) -> Vec<MetadataEntry> {
        self.entries
            .iter()
            .map(|(index, value)| MetadataEntry {
                index: *index,
                value: value.clone(),
            })
            .collect()
    }

    pub const fn has_change_notifications_enabled(&self) -> bool {
        self.change_notifications_are_enabled
    }

    pub fn set_change_notifications_enabled(&mut self, change_notifications_are_enabled: bool) {
        self.change_notifications_are_enabled = change_notifications_are_enabled;
    }

    pub fn drain_dirty_entries(&mut self) -> Vec<MetadataEntry> {
        if !self.change_notifications_are_enabled {
            return Vec::new();
        }
        std::mem::take(&mut self.dirty_entries)
            .into_iter()
            .map(|(index, value)| MetadataEntry { index, value })
            .collect()
    }
}
