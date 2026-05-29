use crate::Identifier;
use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DamageResistant {
    damage_type_tag: Identifier,
}

impl DamageResistant {
    #[must_use]
    pub const fn new(damage_type_tag: Identifier) -> Self {
        Self { damage_type_tag }
    }

    #[must_use]
    pub const fn damage_type_tag(&self) -> &Identifier {
        &self.damage_type_tag
    }
}

impl DataComponentValue for DamageResistant {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "types".to_string(),
            Nbt::String(format!("#{}", self.damage_type_tag)),
        );
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let damage_type_tag = string_field(compound, "types")?;
        Some(Self {
            damage_type_tag: damage_type_tag
                .strip_prefix('#')
                .unwrap_or(&damage_type_tag)
                .parse()
                .ok()?,
        })
    }
}
