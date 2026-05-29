use crate::data_components::UnitComponent;
use crate::{Identifier, ItemStack};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataComponentType<T> {
    id: i32,
    key: Identifier,
    marker: PhantomData<T>,
}

impl<T> DataComponentType<T> {
    #[must_use]
    pub const fn new(id: i32, key: Identifier) -> Self {
        Self {
            id,
            key,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub const fn key(&self) -> &Identifier {
        &self.key
    }
}

pub trait DataComponentValue: Clone + PartialEq {
    fn to_component_nbt(&self) -> Nbt;
    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataComponentDescriptor {
    id: i32,
    key: Identifier,
    is_synced: bool,
    is_serialized: bool,
}

impl DataComponentDescriptor {
    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub const fn key(&self) -> &Identifier {
        &self.key
    }

    #[must_use]
    pub const fn is_synced(&self) -> bool {
        self.is_synced
    }

    #[must_use]
    pub const fn is_serialized(&self) -> bool {
        self.is_serialized
    }

    #[must_use]
    pub fn from_id(id: i32) -> Option<Self> {
        DATA_COMPONENTS
            .iter()
            .find(|component| component.id == id)
            .cloned()
    }

    #[must_use]
    pub fn from_key(key: &str) -> Option<Self> {
        DATA_COMPONENTS
            .iter()
            .find(|component| component.key.to_string() == key || component.key.path == key)
            .cloned()
    }

    #[must_use]
    pub fn values() -> Vec<Self> {
        DATA_COMPONENTS.to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeDataComponentMapError {
    UnknownComponentKey { component_key: String },
}

impl std::fmt::Display for DecodeDataComponentMapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownComponentKey { component_key } => {
                write!(formatter, "unknown data component: {component_key}")
            }
        }
    }
}

impl std::error::Error for DecodeDataComponentMapError {}

#[derive(Clone, Debug, PartialEq)]
enum DataComponentPatch {
    Present(Nbt),
    Removed,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataComponentMap {
    components: BTreeMap<i32, DataComponentPatch>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataComponentMapBuilder {
    components: DataComponentMap,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataComponentPatchBuilder {
    components: DataComponentMap,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataComponentEntry {
    pub component_id: i32,
    pub component_key: Identifier,
    pub component_nbt: Nbt,
}

impl DataComponentMap {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn builder() -> DataComponentMapBuilder {
        DataComponentMapBuilder::new()
    }

    #[must_use]
    pub fn patch_builder() -> DataComponentPatchBuilder {
        DataComponentPatchBuilder::new()
    }

    #[must_use]
    pub fn common_item_components() -> Self {
        Self::new()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    #[must_use]
    pub fn has_component<T>(&self, component: DataComponentType<T>) -> bool {
        self.has_without_fallback(component)
    }

    #[must_use]
    pub fn get_component<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        self.get_without_fallback(component)
    }

    pub fn set<T>(&mut self, component: DataComponentType<T>, value: T)
    where
        T: DataComponentValue,
    {
        self.components.insert(
            component.id,
            DataComponentPatch::Present(value.to_component_nbt()),
        );
    }

    #[must_use]
    pub fn with<T>(&self, component: DataComponentType<T>, value: T) -> Self
    where
        T: DataComponentValue,
    {
        let mut components = self.clone();
        components.set(component, value);
        components
    }

    #[must_use]
    pub fn remove<T>(&self, component: DataComponentType<T>) -> Self {
        let mut components = self.clone();
        components
            .components
            .insert(component.id, DataComponentPatch::Removed);
        components
    }

    #[must_use]
    pub fn has<T>(&self, prototype: &DataComponentMap, component: DataComponentType<T>) -> bool {
        match self.components.get(&component.id) {
            Some(DataComponentPatch::Present(_)) => true,
            Some(DataComponentPatch::Removed) => false,
            None => prototype.has_without_fallback(component),
        }
    }

    #[must_use]
    pub fn get<T>(&self, prototype: &DataComponentMap, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        match self.components.get(&component.id) {
            Some(DataComponentPatch::Present(component_nbt)) => {
                T::from_component_nbt(component_nbt)
            }
            Some(DataComponentPatch::Removed) => None,
            None => prototype.get_without_fallback(component),
        }
    }

    #[must_use]
    pub fn get_or<T>(
        &self,
        prototype: &DataComponentMap,
        component: DataComponentType<T>,
        default_value: T,
    ) -> T
    where
        T: DataComponentValue,
    {
        self.get(prototype, component).unwrap_or(default_value)
    }

    #[must_use]
    pub fn diff(prototype: &DataComponentMap, patch: &DataComponentMap) -> Self {
        let components = patch
            .components
            .iter()
            .filter_map(|(component_id, component_patch)| {
                let prototype_patch = prototype.components.get(component_id);
                match (component_patch, prototype_patch) {
                    (DataComponentPatch::Removed, None) => None,
                    (
                        DataComponentPatch::Present(component_nbt),
                        Some(DataComponentPatch::Present(prototype_nbt)),
                    ) if component_nbt == prototype_nbt => None,
                    _ => Some((*component_id, component_patch.clone())),
                }
            })
            .collect();
        Self { components }
    }

    #[must_use]
    pub fn entries(&self) -> Vec<DataComponentEntry> {
        self.components
            .iter()
            .filter_map(|(component_id, component_patch)| match component_patch {
                DataComponentPatch::Present(component_nbt) => {
                    DataComponentDescriptor::from_id(*component_id).map(|component| {
                        DataComponentEntry {
                            component_id: *component_id,
                            component_key: component.key,
                            component_nbt: component_nbt.clone(),
                        }
                    })
                }
                DataComponentPatch::Removed => None,
            })
            .collect()
    }

    #[must_use]
    pub fn removed_component_ids(&self) -> Vec<i32> {
        self.components
            .iter()
            .filter_map(|(component_id, component_patch)| match component_patch {
                DataComponentPatch::Present(_) => None,
                DataComponentPatch::Removed => Some(*component_id),
            })
            .collect()
    }

    #[must_use]
    pub fn to_builder(&self) -> DataComponentMapBuilder {
        DataComponentMapBuilder {
            components: self.clone(),
        }
    }

    #[must_use]
    pub fn to_patch_builder(&self) -> DataComponentPatchBuilder {
        DataComponentPatchBuilder {
            components: self.clone(),
        }
    }

    #[must_use]
    pub fn to_nbt_patch(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        for (component_id, component_patch) in &self.components {
            if let Some(component) = DataComponentDescriptor::from_id(*component_id) {
                match component_patch {
                    DataComponentPatch::Present(component_nbt) => {
                        compound.insert(component.key.to_string(), component_nbt.clone());
                    }
                    DataComponentPatch::Removed => {
                        compound.insert(format!("!{}", component.key), NbtCompound::new());
                    }
                }
            }
        }
        compound
    }

    pub fn from_nbt_patch(compound: NbtCompound) -> Result<Self, DecodeDataComponentMapError> {
        Self::decode_nbt_compound(compound, DecodeDataComponentMapMode::Patch)
    }

    pub fn from_nbt_map(compound: NbtCompound) -> Result<Self, DecodeDataComponentMapError> {
        Self::decode_nbt_compound(compound, DecodeDataComponentMapMode::Absolute)
    }

    pub fn set_nbt_by_key(
        &mut self,
        component_key: &str,
        component_nbt: Nbt,
    ) -> Result<(), DecodeDataComponentMapError> {
        let component = DataComponentDescriptor::from_key(component_key).ok_or_else(|| {
            DecodeDataComponentMapError::UnknownComponentKey {
                component_key: component_key.to_string(),
            }
        })?;
        self.components
            .insert(component.id, DataComponentPatch::Present(component_nbt));
        Ok(())
    }

    fn decode_nbt_compound(
        compound: NbtCompound,
        mode: DecodeDataComponentMapMode,
    ) -> Result<Self, DecodeDataComponentMapError> {
        compound.0.into_iter().try_fold(
            Self::new(),
            |mut components, (component_key, component_nbt)| {
                let component_was_removed = component_key.starts_with('!');
                let normalized_component_key =
                    component_key.strip_prefix('!').unwrap_or(&component_key);
                let component = DataComponentDescriptor::from_key(normalized_component_key)
                    .ok_or_else(|| DecodeDataComponentMapError::UnknownComponentKey {
                        component_key: normalized_component_key.to_string(),
                    })?;

                if component_was_removed && mode == DecodeDataComponentMapMode::Absolute {
                    return Ok(components);
                }

                let patch = match component_was_removed {
                    true => DataComponentPatch::Removed,
                    false => DataComponentPatch::Present(component_nbt),
                };
                components.components.insert(component.id, patch);
                Ok(components)
            },
        )
    }

    fn has_without_fallback<T>(&self, component: DataComponentType<T>) -> bool {
        matches!(
            self.components.get(&component.id),
            Some(DataComponentPatch::Present(_))
        )
    }

    fn get_without_fallback<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        match self.components.get(&component.id) {
            Some(DataComponentPatch::Present(component_nbt)) => {
                T::from_component_nbt(component_nbt)
            }
            Some(DataComponentPatch::Removed) | None => None,
        }
    }
}

impl DataComponentMapBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T>(&mut self, component: DataComponentType<T>, value: T) -> &mut Self
    where
        T: DataComponentValue,
    {
        self.components.set(component, value);
        self
    }

    pub fn set_unit(&mut self, component: DataComponentType<UnitComponent>) -> &mut Self {
        self.set(component, UnitComponent)
    }

    #[must_use]
    pub fn has<T>(&self, component: DataComponentType<T>) -> bool {
        self.components.has_component(component)
    }

    #[must_use]
    pub fn get<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        self.components.get_component(component)
    }

    #[must_use]
    pub fn build(&self) -> DataComponentMap {
        self.components.clone()
    }
}

impl DataComponentPatchBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T>(&mut self, component: DataComponentType<T>, value: T) -> &mut Self
    where
        T: DataComponentValue,
    {
        self.components.set(component, value);
        self
    }

    pub fn set_unit(&mut self, component: DataComponentType<UnitComponent>) -> &mut Self {
        self.set(component, UnitComponent)
    }

    pub fn remove<T>(&mut self, component: DataComponentType<T>) -> &mut Self {
        self.components = self.components.remove(component);
        self
    }

    #[must_use]
    pub fn has<T>(&self, component: DataComponentType<T>) -> bool {
        self.components.has_component(component)
    }

    #[must_use]
    pub fn get<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        self.components.get_component(component)
    }

    #[must_use]
    pub fn build(&self) -> DataComponentMap {
        self.components.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DecodeDataComponentMapMode {
    Patch,
    Absolute,
}

const DATA_COMPONENTS: &[DataComponentDescriptor] = &[
    DataComponentDescriptor {
        id: 0,
        key: Identifier::vanilla_static("custom_data"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 1,
        key: Identifier::vanilla_static("max_stack_size"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 2,
        key: Identifier::vanilla_static("max_damage"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 3,
        key: Identifier::vanilla_static("damage"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 4,
        key: Identifier::vanilla_static("unbreakable"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 5,
        key: Identifier::vanilla_static("use_effects"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 6,
        key: Identifier::vanilla_static("custom_name"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 7,
        key: Identifier::vanilla_static("minimum_attack_charge"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 8,
        key: Identifier::vanilla_static("damage_type"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 9,
        key: Identifier::vanilla_static("item_name"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 10,
        key: Identifier::vanilla_static("item_model"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 11,
        key: Identifier::vanilla_static("lore"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 12,
        key: Identifier::vanilla_static("rarity"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 13,
        key: Identifier::vanilla_static("enchantments"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 14,
        key: Identifier::vanilla_static("can_place_on"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 15,
        key: Identifier::vanilla_static("can_break"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 16,
        key: Identifier::vanilla_static("attribute_modifiers"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 17,
        key: Identifier::vanilla_static("custom_model_data"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 18,
        key: Identifier::vanilla_static("tooltip_display"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 19,
        key: Identifier::vanilla_static("repair_cost"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 20,
        key: Identifier::vanilla_static("creative_slot_lock"),
        is_synced: true,
        is_serialized: false,
    },
    DataComponentDescriptor {
        id: 21,
        key: Identifier::vanilla_static("enchantment_glint_override"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 22,
        key: Identifier::vanilla_static("intangible_projectile"),
        is_synced: false,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 23,
        key: Identifier::vanilla_static("food"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 24,
        key: Identifier::vanilla_static("consumable"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 25,
        key: Identifier::vanilla_static("use_remainder"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 26,
        key: Identifier::vanilla_static("use_cooldown"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 27,
        key: Identifier::vanilla_static("damage_resistant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 28,
        key: Identifier::vanilla_static("tool"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 29,
        key: Identifier::vanilla_static("weapon"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 30,
        key: Identifier::vanilla_static("attack_range"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 31,
        key: Identifier::vanilla_static("enchantable"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 32,
        key: Identifier::vanilla_static("equippable"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 33,
        key: Identifier::vanilla_static("repairable"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 34,
        key: Identifier::vanilla_static("glider"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 35,
        key: Identifier::vanilla_static("tooltip_style"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 36,
        key: Identifier::vanilla_static("death_protection"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 37,
        key: Identifier::vanilla_static("blocks_attacks"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 38,
        key: Identifier::vanilla_static("piercing_weapon"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 39,
        key: Identifier::vanilla_static("kinetic_weapon"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 40,
        key: Identifier::vanilla_static("swing_animation"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 41,
        key: Identifier::vanilla_static("stored_enchantments"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 42,
        key: Identifier::vanilla_static("dyed_color"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 43,
        key: Identifier::vanilla_static("map_color"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 44,
        key: Identifier::vanilla_static("map_id"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 45,
        key: Identifier::vanilla_static("map_decorations"),
        is_synced: false,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 46,
        key: Identifier::vanilla_static("map_post_processing"),
        is_synced: true,
        is_serialized: false,
    },
    DataComponentDescriptor {
        id: 47,
        key: Identifier::vanilla_static("charged_projectiles"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 48,
        key: Identifier::vanilla_static("bundle_contents"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 49,
        key: Identifier::vanilla_static("potion_contents"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 50,
        key: Identifier::vanilla_static("potion_duration_scale"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 51,
        key: Identifier::vanilla_static("suspicious_stew_effects"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 52,
        key: Identifier::vanilla_static("writable_book_content"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 53,
        key: Identifier::vanilla_static("written_book_content"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 54,
        key: Identifier::vanilla_static("trim"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 55,
        key: Identifier::vanilla_static("debug_stick_state"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 56,
        key: Identifier::vanilla_static("entity_data"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 57,
        key: Identifier::vanilla_static("bucket_entity_data"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 58,
        key: Identifier::vanilla_static("block_entity_data"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 59,
        key: Identifier::vanilla_static("instrument"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 60,
        key: Identifier::vanilla_static("provides_trim_material"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 61,
        key: Identifier::vanilla_static("ominous_bottle_amplifier"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 62,
        key: Identifier::vanilla_static("jukebox_playable"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 63,
        key: Identifier::vanilla_static("provides_banner_patterns"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 64,
        key: Identifier::vanilla_static("recipes"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 65,
        key: Identifier::vanilla_static("lodestone_tracker"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 66,
        key: Identifier::vanilla_static("firework_explosion"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 67,
        key: Identifier::vanilla_static("fireworks"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 68,
        key: Identifier::vanilla_static("profile"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 69,
        key: Identifier::vanilla_static("note_block_sound"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 70,
        key: Identifier::vanilla_static("banner_patterns"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 71,
        key: Identifier::vanilla_static("base_color"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 72,
        key: Identifier::vanilla_static("pot_decorations"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 73,
        key: Identifier::vanilla_static("container"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 74,
        key: Identifier::vanilla_static("block_state"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 75,
        key: Identifier::vanilla_static("bees"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 76,
        key: Identifier::vanilla_static("lock"),
        is_synced: false,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 77,
        key: Identifier::vanilla_static("container_loot"),
        is_synced: false,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 78,
        key: Identifier::vanilla_static("break_sound"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 79,
        key: Identifier::vanilla_static("villager/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 80,
        key: Identifier::vanilla_static("wolf/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 81,
        key: Identifier::vanilla_static("wolf/sound_variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 82,
        key: Identifier::vanilla_static("wolf/collar"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 83,
        key: Identifier::vanilla_static("fox/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 84,
        key: Identifier::vanilla_static("salmon/size"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 85,
        key: Identifier::vanilla_static("parrot/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 86,
        key: Identifier::vanilla_static("tropical_fish/pattern"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 87,
        key: Identifier::vanilla_static("tropical_fish/base_color"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 88,
        key: Identifier::vanilla_static("tropical_fish/pattern_color"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 89,
        key: Identifier::vanilla_static("mooshroom/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 90,
        key: Identifier::vanilla_static("rabbit/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 91,
        key: Identifier::vanilla_static("pig/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 92,
        key: Identifier::vanilla_static("cow/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 93,
        key: Identifier::vanilla_static("chicken/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 94,
        key: Identifier::vanilla_static("zombie_nautilus/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 95,
        key: Identifier::vanilla_static("frog/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 96,
        key: Identifier::vanilla_static("horse/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 97,
        key: Identifier::vanilla_static("painting/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 98,
        key: Identifier::vanilla_static("llama/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 99,
        key: Identifier::vanilla_static("axolotl/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 100,
        key: Identifier::vanilla_static("cat/variant"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 101,
        key: Identifier::vanilla_static("cat/collar"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 102,
        key: Identifier::vanilla_static("sheep/color"),
        is_synced: true,
        is_serialized: true,
    },
    DataComponentDescriptor {
        id: 103,
        key: Identifier::vanilla_static("shulker/color"),
        is_synced: true,
        is_serialized: true,
    },
];

impl DataComponentValue for i32 {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Int(*self)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Int(value) => Some(*value),
            Nbt::Long(value) => i32::try_from(*value).ok(),
            _ => None,
        }
    }
}

impl DataComponentValue for bool {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Byte(i8::from(*self))
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Byte(value) => Some(*value != 0),
            _ => None,
        }
    }
}

impl DataComponentValue for f32 {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Float(*self)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Float(value) => Some(*value),
            Nbt::Double(value) => Some(*value as f32),
            _ => None,
        }
    }
}

impl DataComponentValue for String {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.clone())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl DataComponentValue for Identifier {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.to_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => value.parse().ok(),
            _ => None,
        }
    }
}

impl DataComponentValue for Vec<Identifier> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(|identifier| Nbt::String(identifier.to_string()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(identifiers) => identifiers
                .iter()
                .map(|identifier| match identifier {
                    Nbt::String(value) => value.parse().ok(),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl DataComponentValue for Vec<String> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .cloned()
                .map(Nbt::String)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(values) => values
                .iter()
                .map(|value| match value {
                    Nbt::String(value) => Some(value.clone()),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl DataComponentValue for ItemStack {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Compound(self.to_item_nbt())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Compound(compound) => Self::from_item_nbt(compound.clone()),
            _ => None,
        }
    }
}

impl DataComponentValue for Vec<ItemStack> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(ItemStack::to_component_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(item_stacks) => item_stacks
                .iter()
                .map(ItemStack::from_component_nbt)
                .collect(),
            _ => None,
        }
    }
}

impl DataComponentValue for Nbt {
    fn to_component_nbt(&self) -> Nbt {
        self.clone()
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        Some(component_nbt.clone())
    }
}

impl DataComponentValue for NbtCompound {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Compound(self.clone())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Compound(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl DataComponentValue for TextComponent {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.to_json_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => serde_json::from_str(value).ok(),
            _ => None,
        }
    }
}

impl DataComponentValue for Vec<TextComponent> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(|component| Nbt::String(component.to_json_string()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(components) => components
                .iter()
                .map(|component| match component {
                    Nbt::String(value) => serde_json::from_str(value).ok(),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}
