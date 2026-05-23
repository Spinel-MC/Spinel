use crate::data_components::vanilla_components::{
    CUSTOM_DATA, CUSTOM_NAME, DAMAGE, ENCHANTMENT_GLINT_OVERRIDE, ITEM_MODEL, LORE, MAX_DAMAGE,
    MAX_STACK_SIZE,
};
use crate::{DataComponentMap, DataComponentType, DataComponentValue, ItemStackBuilder, Material};
use spinel_nbt::{NbtCompound, Tag, TagReadable};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemStack {
    material: Material,
    amount: i32,
    component_patch: DataComponentMap,
}

impl ItemStack {
    pub fn air() -> Self {
        Self::from_parts(Material::AIR, 0, DataComponentMap::new())
    }

    pub fn of(material: Material) -> Self {
        Self::from_parts(material, 1, DataComponentMap::new())
    }

    pub fn builder(material: Material) -> ItemStackBuilder {
        ItemStackBuilder::new(material)
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn component_patch(&self) -> &DataComponentMap {
        &self.component_patch
    }

    pub fn to_builder(&self) -> ItemStackBuilder {
        ItemStackBuilder::from_item_stack(self.clone())
    }

    pub fn with_material(&self, material: Material) -> Self {
        let amount = match self.amount {
            0 => 1,
            amount => amount,
        };
        Self::from_parts(material, amount, self.component_patch.clone())
    }

    pub fn with_amount(&self, amount: i32) -> Self {
        Self::from_parts(self.material.clone(), amount, self.component_patch.clone())
    }

    pub fn transform_amount(&self, operator: impl FnOnce(i32) -> i32) -> Self {
        self.with_amount(operator(self.amount))
    }

    pub fn has<T>(&self, component: DataComponentType<T>) -> bool {
        self.component_patch
            .has(&self.material.prototype(), component)
    }

    pub fn get<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        self.component_patch
            .get(&self.material.prototype(), component)
    }

    pub fn get_or<T>(&self, component: DataComponentType<T>, default_value: T) -> T
    where
        T: DataComponentValue,
    {
        self.component_patch
            .get_or(&self.material.prototype(), component, default_value)
    }

    pub fn with<T>(&self, component: DataComponentType<T>, value: T) -> Self
    where
        T: DataComponentValue,
    {
        Self::from_parts(
            self.material.clone(),
            self.amount,
            self.component_patch.with(component, value),
        )
    }

    pub fn with_component_by<T>(
        &self,
        component: DataComponentType<T>,
        operator: impl FnOnce(T) -> T,
    ) -> Self
    where
        T: DataComponentValue,
    {
        self.get(component.clone())
            .map(|component_value| self.with(component, operator(component_value)))
            .unwrap_or_else(|| self.clone())
    }

    pub fn without<T>(&self, component: DataComponentType<T>) -> Self
    where
        T: DataComponentValue,
    {
        if !self.has(component.clone()) {
            return self.clone();
        }
        Self::from_parts(
            self.material.clone(),
            self.amount,
            self.component_patch.remove(component),
        )
    }

    pub fn with_tag<T>(&self, tag: &Tag<T>, value: Option<T>) -> Self {
        let mut custom_data = self.get_or(CUSTOM_DATA, NbtCompound::new());
        tag.write(&mut custom_data, value);
        self.with(CUSTOM_DATA, custom_data)
    }

    pub fn without_tag<T>(&self, tag: &Tag<T>) -> Self {
        self.with_tag(tag, None)
    }

    pub fn with_custom_name(&self, custom_name: TextComponent) -> Self {
        self.with(CUSTOM_NAME, custom_name)
    }

    pub fn with_lore(&self, lore: Vec<TextComponent>) -> Self {
        self.with(LORE, lore)
    }

    pub fn with_item_model(&self, item_model: impl Into<String>) -> Self {
        self.with(ITEM_MODEL, item_model.into())
    }

    pub fn with_glowing(&self, glowing: bool) -> Self {
        self.with(ENCHANTMENT_GLINT_OVERRIDE, glowing)
    }

    pub fn without_extra_tooltip(&self) -> Self {
        self.clone()
    }

    pub fn with_max_stack_size(&self, max_stack_size: i32) -> Self {
        self.with(MAX_STACK_SIZE, max_stack_size)
    }

    pub fn consume(&self, amount: i32) -> Self {
        self.with_amount(self.amount - amount)
    }

    pub fn damage(&self, amount: i32) -> Self {
        let Some(damage) = self.get(DAMAGE) else {
            return self.clone();
        };
        let next_damage = damage + amount;
        if self.max_damage_reached(next_damage) {
            return Self::air();
        }
        self.with(DAMAGE, next_damage)
    }

    pub fn is_air(&self) -> bool {
        self.material == Material::AIR
    }

    pub fn is_similar(&self, item_stack: &ItemStack) -> bool {
        self.material == item_stack.material && self.component_patch == item_stack.component_patch
    }

    pub fn max_stack_size(&self) -> i32 {
        self.get_or(MAX_STACK_SIZE, 64)
    }
}

impl ItemStack {
    pub(crate) fn from_parts(
        material: Material,
        amount: i32,
        component_patch: DataComponentMap,
    ) -> Self {
        if material == Material::AIR || amount <= 0 {
            return Self {
                material: Material::AIR,
                amount: 0,
                component_patch: DataComponentMap::new(),
            };
        }
        let component_patch = DataComponentMap::diff(&material.prototype(), &component_patch);
        Self {
            material,
            amount,
            component_patch,
        }
    }

    fn max_damage_reached(&self, damage: i32) -> bool {
        self.get(MAX_DAMAGE)
            .is_some_and(|max_damage| damage >= max_damage)
    }
}

impl TagReadable for ItemStack {
    fn get_tag<T>(&self, tag: &Tag<T>) -> Option<T> {
        self.get(CUSTOM_DATA)
            .and_then(|custom_data| tag.read(&custom_data))
    }
}

impl Default for ItemStack {
    fn default() -> Self {
        Self::air()
    }
}
