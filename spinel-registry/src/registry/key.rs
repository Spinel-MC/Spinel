use crate::Identifier;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct RegistryKey<T> {
    key: Identifier,
    _marker: PhantomData<T>,
}

impl<T> RegistryKey<T> {
    #[must_use]
    pub const fn vanilla_static(path: &'static str) -> Self {
        Self {
            key: Identifier::vanilla_static(path),
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn new(key: Identifier) -> Self {
        Self {
            key,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn key(&self) -> &Identifier {
        &self.key
    }
}

impl<T> Clone for RegistryKey<T> {
    fn clone(&self) -> Self {
        Self::new(self.key.clone())
    }
}

impl<T> PartialEq for RegistryKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for RegistryKey<T> {}

impl<T> PartialOrd for RegistryKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for RegistryKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
impl<T> Hash for RegistryKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
