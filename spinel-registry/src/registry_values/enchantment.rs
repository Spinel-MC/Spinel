use crate::data_components::{DataComponentMap, RegistryTagReference};
use crate::{EquipmentSlotGroup, RegistryCodec};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Debug, PartialEq)]
pub struct Enchantment {
    description: TextComponent,
    exclusive_set: RegistryTagReference,
    supported_items: RegistryTagReference,
    primary_items: Option<RegistryTagReference>,
    weight: i32,
    max_level: i32,
    min_cost: EnchantmentCost,
    max_cost: EnchantmentCost,
    anvil_cost: i32,
    slots: Vec<EquipmentSlotGroup>,
    effects: DataComponentMap,
    raw_registry_nbt: Option<NbtCompound>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnchantmentCost {
    base: i32,
    per_level_above_first: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnchantmentBuilder {
    description: TextComponent,
    exclusive_set: RegistryTagReference,
    supported_items: RegistryTagReference,
    primary_items: Option<RegistryTagReference>,
    weight: i32,
    max_level: i32,
    min_cost: EnchantmentCost,
    max_cost: EnchantmentCost,
    anvil_cost: i32,
    slots: Vec<EquipmentSlotGroup>,
    effects: DataComponentMap,
}

impl Enchantment {
    #[must_use]
    pub fn new(
        description: TextComponent,
        exclusive_set: RegistryTagReference,
        supported_items: RegistryTagReference,
        primary_items: Option<RegistryTagReference>,
        weight: i32,
        max_level: i32,
        min_cost: EnchantmentCost,
        max_cost: EnchantmentCost,
        anvil_cost: i32,
        slots: Vec<EquipmentSlotGroup>,
        effects: DataComponentMap,
    ) -> Self {
        Self {
            description,
            exclusive_set,
            supported_items,
            primary_items,
            weight,
            max_level,
            min_cost,
            max_cost,
            anvil_cost,
            slots,
            effects,
            raw_registry_nbt: None,
        }
    }

    #[must_use]
    pub fn builder() -> EnchantmentBuilder {
        EnchantmentBuilder::default()
    }

    #[must_use]
    pub const fn get_description(&self) -> &TextComponent {
        &self.description
    }

    #[must_use]
    pub const fn get_exclusive_set(&self) -> &RegistryTagReference {
        &self.exclusive_set
    }

    #[must_use]
    pub const fn get_supported_items(&self) -> &RegistryTagReference {
        &self.supported_items
    }

    #[must_use]
    pub const fn get_primary_items(&self) -> Option<&RegistryTagReference> {
        self.primary_items.as_ref()
    }

    #[must_use]
    pub const fn get_weight(&self) -> i32 {
        self.weight
    }

    #[must_use]
    pub const fn get_max_level(&self) -> i32 {
        self.max_level
    }

    #[must_use]
    pub const fn get_min_cost(&self) -> EnchantmentCost {
        self.min_cost
    }

    #[must_use]
    pub const fn get_max_cost(&self) -> EnchantmentCost {
        self.max_cost
    }

    #[must_use]
    pub const fn get_anvil_cost(&self) -> i32 {
        self.anvil_cost
    }

    #[must_use]
    pub fn get_slots(&self) -> &[EquipmentSlotGroup] {
        &self.slots
    }

    #[must_use]
    pub const fn get_effects(&self) -> &DataComponentMap {
        &self.effects
    }

    #[must_use]
    pub fn raw(data: NbtCompound) -> Self {
        Self {
            raw_registry_nbt: Some(data),
            ..Self::default()
        }
    }
}

impl Default for Enchantment {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Default for EnchantmentCost {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl Default for EnchantmentBuilder {
    fn default() -> Self {
        Self {
            description: TextComponent::empty(),
            exclusive_set: RegistryTagReference::empty(),
            supported_items: RegistryTagReference::empty(),
            primary_items: None,
            weight: 1,
            max_level: 1,
            min_cost: EnchantmentCost::default(),
            max_cost: EnchantmentCost::default(),
            anvil_cost: 0,
            slots: Vec::new(),
            effects: DataComponentMap::new(),
        }
    }
}

impl EnchantmentCost {
    #[must_use]
    pub const fn new(base: i32, per_level_above_first: i32) -> Self {
        Self {
            base,
            per_level_above_first,
        }
    }

    #[must_use]
    pub const fn get_base(&self) -> i32 {
        self.base
    }

    #[must_use]
    pub const fn get_per_level_above_first(&self) -> i32 {
        self.per_level_above_first
    }

    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("base".to_owned(), Nbt::Int(self.base));
        compound.insert(
            "per_level_above_first".to_owned(),
            Nbt::Int(self.per_level_above_first),
        );
        Nbt::Compound(compound)
    }
}

impl EnchantmentBuilder {
    #[must_use]
    pub fn description(mut self, description: TextComponent) -> Self {
        self.description = description;
        self
    }

    #[must_use]
    pub fn exclusive_set(mut self, exclusive_set: RegistryTagReference) -> Self {
        self.exclusive_set = exclusive_set;
        self
    }

    #[must_use]
    pub fn supported_items(mut self, supported_items: RegistryTagReference) -> Self {
        self.supported_items = supported_items;
        self
    }

    #[must_use]
    pub fn primary_items(mut self, primary_items: RegistryTagReference) -> Self {
        self.primary_items = Some(primary_items);
        self
    }

    #[must_use]
    pub const fn weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }

    #[must_use]
    pub const fn max_level(mut self, max_level: i32) -> Self {
        self.max_level = max_level;
        self
    }

    #[must_use]
    pub const fn min_cost(mut self, min_cost: EnchantmentCost) -> Self {
        self.min_cost = min_cost;
        self
    }

    #[must_use]
    pub const fn min_cost_values(mut self, base: i32, per_level_above_first: i32) -> Self {
        self.min_cost = EnchantmentCost::new(base, per_level_above_first);
        self
    }

    #[must_use]
    pub const fn max_cost(mut self, max_cost: EnchantmentCost) -> Self {
        self.max_cost = max_cost;
        self
    }

    #[must_use]
    pub const fn max_cost_values(mut self, base: i32, per_level_above_first: i32) -> Self {
        self.max_cost = EnchantmentCost::new(base, per_level_above_first);
        self
    }

    #[must_use]
    pub const fn anvil_cost(mut self, anvil_cost: i32) -> Self {
        self.anvil_cost = anvil_cost;
        self
    }

    #[must_use]
    pub fn slots(mut self, slots: Vec<EquipmentSlotGroup>) -> Self {
        self.slots = slots;
        self
    }

    #[must_use]
    pub fn effects(mut self, effects: DataComponentMap) -> Self {
        self.effects = effects;
        self
    }

    #[must_use]
    pub fn build(self) -> Enchantment {
        Enchantment::new(
            self.description,
            self.exclusive_set,
            self.supported_items,
            self.primary_items,
            self.weight,
            self.max_level,
            self.min_cost,
            self.max_cost,
            self.anvil_cost,
            self.slots,
            self.effects,
        )
    }
}

impl RegistryCodec for Enchantment {
    fn registry_nbt(&self) -> NbtCompound {
        if let Some(raw_registry_nbt) = &self.raw_registry_nbt {
            return raw_registry_nbt.clone();
        }
        let mut compound = NbtCompound::new();
        compound.insert(
            "description".to_owned(),
            Nbt::Compound(self.description.to_nbt_compound()),
        );
        if self.exclusive_set != RegistryTagReference::empty() {
            compound.insert("exclusive_set".to_owned(), self.exclusive_set.to_nbt());
        }
        compound.insert("supported_items".to_owned(), self.supported_items.to_nbt());
        if let Some(primary_items) = &self.primary_items {
            compound.insert("primary_items".to_owned(), primary_items.to_nbt());
        }
        compound.insert("weight".to_owned(), Nbt::Int(self.weight));
        compound.insert("max_level".to_owned(), Nbt::Int(self.max_level));
        compound.insert("min_cost".to_owned(), self.min_cost.to_nbt());
        compound.insert("max_cost".to_owned(), self.max_cost.to_nbt());
        compound.insert("anvil_cost".to_owned(), Nbt::Int(self.anvil_cost));
        compound.insert(
            "slots".to_owned(),
            Nbt::List(
                self.slots
                    .iter()
                    .map(|slot| Nbt::String(slot.nbt_name().to_owned()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        );
        if !self.effects.is_empty() {
            compound.insert(
                "effects".to_owned(),
                Nbt::Compound(self.effects.to_nbt_patch()),
            );
        }
        compound
    }
}
