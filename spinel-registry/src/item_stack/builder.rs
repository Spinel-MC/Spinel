use crate::{DataComponentType, DataComponentValue, ItemStack, Material};
use spinel_nbt::Tag;
use spinel_utils::component::text::TextComponent;

pub struct ItemStackBuilder {
    item_stack: ItemStack,
}

impl ItemStackBuilder {
    pub fn new(material: Material) -> Self {
        Self {
            item_stack: ItemStack::of(material),
        }
    }

    pub fn from_item_stack(item_stack: ItemStack) -> Self {
        Self { item_stack }
    }

    pub fn amount(&mut self, amount: i32) -> &mut Self {
        self.item_stack = self.item_stack.with_amount(amount);
        self
    }

    pub fn material(&mut self, material: Material) -> &mut Self {
        self.item_stack = self.item_stack.with_material(material);
        self
    }

    pub fn set<T>(&mut self, component: DataComponentType<T>, value: T) -> &mut Self
    where
        T: DataComponentValue,
    {
        self.item_stack = self.item_stack.with(component, value);
        self
    }

    pub fn remove<T>(&mut self, component: DataComponentType<T>) -> &mut Self
    where
        T: DataComponentValue,
    {
        self.item_stack = self.item_stack.without(component);
        self
    }

    pub fn custom_name(&mut self, custom_name: TextComponent) -> &mut Self {
        self.item_stack = self.item_stack.with_custom_name(custom_name);
        self
    }

    pub fn lore(&mut self, lore: Vec<TextComponent>) -> &mut Self {
        self.item_stack = self.item_stack.with_lore(lore);
        self
    }

    pub fn item_model(&mut self, item_model: impl Into<String>) -> &mut Self {
        self.item_stack = self.item_stack.with_item_model(item_model);
        self
    }

    pub fn glowing(&mut self, glowing: bool) -> &mut Self {
        self.item_stack = self.item_stack.with_glowing(glowing);
        self
    }

    pub fn max_stack_size(&mut self, max_stack_size: i32) -> &mut Self {
        self.item_stack = self.item_stack.with_max_stack_size(max_stack_size);
        self
    }

    pub fn hide_extra_tooltip(&mut self) -> &mut Self {
        self.item_stack = self.item_stack.without_extra_tooltip();
        self
    }

    pub fn set_tag<T>(&mut self, tag: &Tag<T>, value: Option<T>) -> &mut Self {
        self.item_stack = self.item_stack.with_tag(tag, value);
        self
    }

    pub fn build(&self) -> ItemStack {
        self.item_stack.clone()
    }
}
