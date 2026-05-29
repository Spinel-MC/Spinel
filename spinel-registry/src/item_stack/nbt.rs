use crate::{DataComponentMap, ItemStack, Material};
use spinel_nbt::{Nbt, NbtCompound};

impl ItemStack {
    pub fn to_item_nbt(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("id".to_string(), self.material().key().to_string());
        compound.insert("count".to_string(), self.amount());
        if !self.component_patch().is_empty() {
            compound.insert(
                "components".to_string(),
                self.component_patch().to_nbt_patch(),
            );
        }
        compound
    }

    pub fn from_item_nbt(compound: NbtCompound) -> Option<Self> {
        let material = match compound.get("id") {
            Some(Nbt::String(key)) => Material::from_key(key)?,
            _ => return None,
        };
        let amount = match compound.get("count") {
            Some(Nbt::Int(amount)) => *amount,
            _ => 1,
        };
        let component_patch = match compound.get("components") {
            Some(Nbt::Compound(component_patch)) => {
                DataComponentMap::from_nbt_patch(component_patch.clone()).ok()?
            }
            _ => DataComponentMap::new(),
        };
        Some(Self::from_parts(material, amount, component_patch))
    }
}
