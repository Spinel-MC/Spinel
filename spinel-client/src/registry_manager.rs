use spinel_nbt::NbtCompound;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub id: String,
    pub data: Option<NbtCompound>,
}

#[derive(Debug, Clone, Default)]
pub struct DynamicRegistries {
    registries: HashMap<String, Vec<RegistryEntry>>,
}

impl DynamicRegistries {
    pub fn new() -> Self {
        Self {
            registries: HashMap::new(),
        }
    }

    pub fn put(&mut self, registry_key: String, entries: Vec<(String, Option<NbtCompound>)>) {
        self.registries
            .entry(registry_key)
            .or_insert_with(Vec::new)
            .extend(
                entries
                    .into_iter()
                    .map(|(id, data)| RegistryEntry { id, data }),
            );
    }

    pub fn get(&self, registry_key: &str) -> Option<&Vec<RegistryEntry>> {
        self.registries.get(registry_key)
    }

    pub fn len(&self) -> usize {
        self.registries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.registries.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct TagRegistry {
    tags: HashMap<String, HashMap<String, spinel_network::types::IntList>>,
}

impl TagRegistry {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    pub fn put(
        &mut self,
        registry_key: String,
        tags: HashMap<String, spinel_network::types::IntList>,
    ) {
        self.tags.insert(registry_key, tags);
    }

    pub fn get(
        &self,
        registry_key: &str,
    ) -> Option<&HashMap<String, spinel_network::types::IntList>> {
        self.tags.get(registry_key)
    }

    pub fn len(&self) -> usize {
        self.tags.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientRegistries {
    pub dynamic_registries: DynamicRegistries,
    pub tags: TagRegistry,
}

impl ClientRegistries {
    pub fn new() -> Self {
        Self {
            dynamic_registries: DynamicRegistries::new(),
            tags: TagRegistry::new(),
        }
    }
}
