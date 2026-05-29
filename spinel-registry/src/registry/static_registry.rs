use crate::{Identifier, RegistryKey};
use std::collections::HashMap;

pub struct StaticRegistry<T> {
    entries: Vec<(RegistryKey<T>, T)>,
    key_to_id: HashMap<Identifier, usize>,
    allows_registering: bool,
}

impl<T> StaticRegistry<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            key_to_id: HashMap::new(),
            allows_registering: true,
        }
    }

    pub fn register(
        &mut self,
        key: RegistryKey<T>,
        value: T,
    ) -> Result<RegistryKey<T>, RegisterStaticError> {
        if !self.allows_registering {
            return Err(RegisterStaticError::Frozen);
        }
        if self.key_to_id.contains_key(key.key()) {
            return Err(RegisterStaticError::DuplicateKey);
        }
        self.key_to_id.insert(key.key().clone(), self.entries.len());
        self.entries.push((key.clone(), value));
        Ok(key)
    }

    #[must_use]
    pub fn get_id(&self, key: &RegistryKey<T>) -> Option<i32> {
        self.key_to_id.get(key.key()).map(|id| *id as i32)
    }

    #[must_use]
    pub fn get(&self, key: &RegistryKey<T>) -> Option<&T> {
        self.key_to_id
            .get(key.key())
            .and_then(|entry_id| self.entries.get(*entry_id))
            .map(|(_key, value)| value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &RegistryKey<T>, &T)> {
        self.entries
            .iter()
            .enumerate()
            .map(|(entry_id, (key, value))| (entry_id, key, value))
    }

    pub fn freeze(&mut self) {
        self.allows_registering = false;
    }
}

impl<T: PartialEq> StaticRegistry<T> {
    #[must_use]
    pub fn key_for(&self, value: &T) -> Option<&RegistryKey<T>> {
        self.entries
            .iter()
            .find(|(_, entry_value)| entry_value == value)
            .map(|(key, _)| key)
    }
}

impl<T> Default for StaticRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterStaticError {
    Frozen,
    DuplicateKey,
}
