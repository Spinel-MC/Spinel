use crate::{DynamicRegistry, Identifier, RegistryKey, StaticRegistry};

pub struct RegistryTags {
    pub registry_name: Identifier,
    pub tags: Vec<RegistryTag>,
}

impl RegistryTags {
    #[must_use]
    pub fn new(registry_name: Identifier, tags: Vec<RegistryTag>) -> Self {
        Self {
            registry_name,
            tags,
        }
    }
}

pub struct RegistryTag {
    pub tag_name: Identifier,
    pub entries: Vec<i32>,
}

impl RegistryTag {
    #[must_use]
    pub fn new(tag_name: Identifier, entries: Vec<i32>) -> Self {
        Self { tag_name, entries }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryTagError {
    InvalidEntry(String),
    MissingEntry {
        registry_name: Identifier,
        tag_name: Identifier,
        entry_name: Identifier,
    },
}

pub(crate) fn dynamic_registry_tags<T>(
    registry: &DynamicRegistry<T>,
    registry_name: Identifier,
    generated_tags: &'static [(&'static str, &'static [&'static str])],
) -> Result<Vec<RegistryTag>, RegistryTagError> {
    generated_tags
        .iter()
        .map(|(tag_name, entries)| {
            dynamic_registry_tag(registry, &registry_name, tag_name, entries)
        })
        .collect()
}

pub(crate) fn static_registry_tags<T>(
    registry: &StaticRegistry<T>,
    registry_name: Identifier,
    generated_tags: &'static [(&'static str, &'static [&'static str])],
) -> Result<Vec<RegistryTag>, RegistryTagError> {
    generated_tags
        .iter()
        .map(|(tag_name, entries)| static_registry_tag(registry, &registry_name, tag_name, entries))
        .collect()
}

fn dynamic_registry_tag<T>(
    registry: &DynamicRegistry<T>,
    registry_name: &Identifier,
    tag_name: &'static str,
    entries: &'static [&'static str],
) -> Result<RegistryTag, RegistryTagError> {
    let tag_name = parse_entry(tag_name)?;
    let entries = entries
        .iter()
        .map(|entry_name| {
            dynamic_registry_tag_entry(registry, registry_name, &tag_name, entry_name)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RegistryTag::new(tag_name, entries))
}

fn static_registry_tag<T>(
    registry: &StaticRegistry<T>,
    registry_name: &Identifier,
    tag_name: &'static str,
    entries: &'static [&'static str],
) -> Result<RegistryTag, RegistryTagError> {
    let tag_name = parse_entry(tag_name)?;
    let entries = entries
        .iter()
        .map(|entry_name| static_registry_tag_entry(registry, registry_name, &tag_name, entry_name))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RegistryTag::new(tag_name, entries))
}

fn dynamic_registry_tag_entry<T>(
    registry: &DynamicRegistry<T>,
    registry_name: &Identifier,
    tag_name: &Identifier,
    entry_name: &'static str,
) -> Result<i32, RegistryTagError> {
    let entry_name = parse_entry(entry_name)?;
    let entry_key = RegistryKey::new(entry_name.clone());
    registry
        .get_id(&entry_key)
        .ok_or_else(|| RegistryTagError::MissingEntry {
            registry_name: registry_name.clone(),
            tag_name: tag_name.clone(),
            entry_name,
        })
}

fn static_registry_tag_entry<T>(
    registry: &StaticRegistry<T>,
    registry_name: &Identifier,
    tag_name: &Identifier,
    entry_name: &'static str,
) -> Result<i32, RegistryTagError> {
    let entry_name = parse_entry(entry_name)?;
    let entry_key = RegistryKey::new(entry_name.clone());
    registry
        .get_id(&entry_key)
        .ok_or_else(|| RegistryTagError::MissingEntry {
            registry_name: registry_name.clone(),
            tag_name: tag_name.clone(),
            entry_name,
        })
}

fn parse_entry(entry_name: &'static str) -> Result<Identifier, RegistryTagError> {
    entry_name
        .parse()
        .map_err(|_| RegistryTagError::InvalidEntry(entry_name.to_string()))
}
