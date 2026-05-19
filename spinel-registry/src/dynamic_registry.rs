use crate::{Identifier, RegistryCodec, RegistryKey};
use spinel_nbt::NbtCompound;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrySource {
    Vanilla,
    Custom,
}

#[derive(Debug)]
pub struct RegistryEntry<T> {
    key: RegistryKey<T>,
    value: T,
    source: RegistrySource,
}

impl<T> RegistryEntry<T> {
    #[must_use]
    pub fn key(&self) -> &RegistryKey<T> {
        &self.key
    }

    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }
}

pub struct DynamicRegistry<T> {
    key: Identifier,
    entries: Vec<RegistryEntry<T>>,
    key_to_id: HashMap<Identifier, usize>,
    allows_registering: bool,
}

impl<T> DynamicRegistry<T> {
    #[must_use]
    pub fn new(key: Identifier) -> Self {
        Self {
            key,
            entries: Vec::new(),
            key_to_id: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(&mut self, key: Identifier, value: T) -> Result<RegistryKey<T>, RegisterError> {
        self.register_with_source(key, value, RegistrySource::Custom)
    }

    pub fn register_vanilla(
        &mut self,
        key: RegistryKey<T>,
        value: T,
    ) -> Result<RegistryKey<T>, RegisterError> {
        self.register_entry(key, value, RegistrySource::Vanilla)
    }

    #[must_use]
    pub fn key(&self) -> &Identifier {
        &self.key
    }

    #[must_use]
    pub fn id_of(&self, key: &RegistryKey<T>) -> Option<i32> {
        self.key_to_id.get(key.key()).map(|id| *id as i32)
    }

    #[must_use]
    pub fn get_id(&self, key: &RegistryKey<T>) -> Option<i32> {
        self.id_of(key)
    }

    #[must_use]
    pub fn get(&self, key: &RegistryKey<T>) -> Option<&T> {
        self.key_to_id
            .get(key.key())
            .and_then(|entry_id| self.entries.get(*entry_id))
            .map(RegistryEntry::value)
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (usize, &RegistryEntry<T>)> {
        self.entries.iter().enumerate()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn freeze(&mut self) {
        self.allows_registering = false;
    }

    fn register_with_source(
        &mut self,
        key: Identifier,
        value: T,
        source: RegistrySource,
    ) -> Result<RegistryKey<T>, RegisterError> {
        let registry_key = RegistryKey::new(key);
        self.register_entry(registry_key, value, source)
    }

    fn register_entry(
        &mut self,
        key: RegistryKey<T>,
        value: T,
        source: RegistrySource,
    ) -> Result<RegistryKey<T>, RegisterError> {
        if !self.allows_registering {
            return Err(RegisterError::Frozen);
        }
        if self.key_to_id.contains_key(key.key()) {
            return Err(RegisterError::DuplicateKey);
        }
        let entry_id = self.entries.len();
        self.key_to_id.insert(key.key().clone(), entry_id);
        self.entries.push(RegistryEntry {
            key: key.clone(),
            value,
            source,
        });
        Ok(key)
    }
}

impl<T> DynamicRegistry<T>
where
    T: RegistryCodec,
{
    #[must_use]
    pub fn registry_packet_entries(
        &self,
        exclude_vanilla: bool,
    ) -> Vec<(Identifier, Option<NbtCompound>)> {
        self.entries
            .iter()
            .map(|entry| {
                let data_is_omitted = exclude_vanilla && entry.source == RegistrySource::Vanilla;
                let local_data = (!data_is_omitted).then(|| entry.value.registry_nbt());
                (entry.key.key().clone(), local_data)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterError {
    Frozen,
    DuplicateKey,
}

#[cfg(test)]
#[path = "dynamic_registry_tests.rs"]
mod dynamic_registry_tests;
