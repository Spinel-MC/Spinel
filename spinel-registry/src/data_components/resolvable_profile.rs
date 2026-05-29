use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResolvableProfile {
    name: Option<String>,
    uuid: Option<String>,
    properties: Vec<GameProfileProperty>,
    texture: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameProfileProperty {
    name: String,
    value: String,
    signature: Option<String>,
}

impl ResolvableProfile {
    #[must_use]
    pub fn new(
        name: Option<String>,
        uuid: Option<String>,
        properties: Vec<GameProfileProperty>,
        texture: Option<String>,
    ) -> Self {
        Self {
            name,
            uuid,
            properties,
            texture,
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub fn uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    #[must_use]
    pub fn properties(&self) -> &[GameProfileProperty] {
        &self.properties
    }

    #[must_use]
    pub fn texture(&self) -> Option<&str> {
        self.texture.as_deref()
    }
}

impl GameProfileProperty {
    #[must_use]
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Self {
            name,
            value,
            signature,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }
}

impl DataComponentValue for ResolvableProfile {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if let Some(name) = &self.name {
            compound.insert("name".to_string(), Nbt::String(name.clone()));
        }
        if let Some(uuid) = &self.uuid {
            compound.insert("id".to_string(), Nbt::String(uuid.clone()));
        }
        if !self.properties.is_empty() {
            compound.insert(
                "properties".to_string(),
                Nbt::List(
                    self.properties
                        .iter()
                        .map(GameProfileProperty::to_nbt)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if let Some(texture) = &self.texture {
            compound.insert("texture".to_string(), Nbt::String(texture.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let properties = match compound.get("properties") {
            Some(Nbt::List(properties)) => properties
                .iter()
                .map(GameProfileProperty::from_nbt)
                .collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
            _ => return None,
        };
        Some(Self {
            name: optional_string(compound, "name")?,
            uuid: optional_uuid(compound, "id")?,
            properties,
            texture: optional_string(compound, "texture")?,
        })
    }
}

impl GameProfileProperty {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("name".to_string(), Nbt::String(self.name.clone()));
        compound.insert("value".to_string(), Nbt::String(self.value.clone()));
        if let Some(signature) = &self.signature {
            compound.insert("signature".to_string(), Nbt::String(signature.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            name: string_field(compound, "name")?,
            value: string_field(compound, "value")?,
            signature: optional_string(compound, "signature")?,
        })
    }
}

fn optional_string(compound: &NbtCompound, name: &str) -> Option<Option<String>> {
    match compound.get(name) {
        Some(_) => Some(Some(string_field(compound, name)?)),
        None => Some(None),
    }
}

fn optional_uuid(compound: &NbtCompound, name: &str) -> Option<Option<String>> {
    match compound.get(name) {
        Some(Nbt::String(value)) => Some(Some(value.clone())),
        Some(Nbt::IntArray(values)) if values.len() == 4 => {
            let mut bytes = [0_u8; 16];
            for (index, value) in values.iter().enumerate() {
                bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_be_bytes());
            }
            Some(Some(format!(
                "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
                u32::from_be_bytes(bytes[0..4].try_into().ok()?),
                u16::from_be_bytes(bytes[4..6].try_into().ok()?),
                u16::from_be_bytes(bytes[6..8].try_into().ok()?),
                u16::from_be_bytes(bytes[8..10].try_into().ok()?),
                u64::from_be_bytes([
                    0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
                ])
            )))
        }
        None => Some(None),
        _ => None,
    }
}
