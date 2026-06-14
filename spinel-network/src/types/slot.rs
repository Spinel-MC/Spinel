use crate::data_type::DataType;
use crate::types::component_changes::ComponentChanges;
use crate::types::var_int::VarIntWrapper;
use spinel_registry::{ItemStack, Material};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Slot {
    pub count: i32,
    pub item_id: i32,
    pub components: ComponentChanges,
}

impl Slot {
    pub fn is_empty(&self) -> bool {
        self.count <= 0
    }

    pub fn from_item_stack(item_stack: &ItemStack) -> Self {
        if item_stack.is_air() {
            return Self::default();
        }
        Self {
            count: item_stack.amount(),
            item_id: item_stack.material().id(),
            components: ComponentChanges::from(item_stack.component_patch()),
        }
    }

    pub fn to_item_stack(&self) -> ItemStack {
        if self.is_empty() {
            return ItemStack::air();
        }
        let Some(material) = Material::from_id(self.item_id) else {
            return ItemStack::air();
        };
        let item_stack = ItemStack::of(material).with_amount(self.count);
        let item_stack = self
            .components
            .custom_data()
            .ok()
            .flatten()
            .map(|custom_data| {
                item_stack.with(
                    spinel_registry::data_components::vanilla_components::CUSTOM_DATA,
                    custom_data,
                )
            })
            .unwrap_or(item_stack);
        self.components
            .custom_name()
            .ok()
            .flatten()
            .map(|custom_name| item_stack.with_custom_name(custom_name))
            .unwrap_or(item_stack)
    }
}

impl DataType for Slot {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        if self.is_empty() {
            VarIntWrapper(0).encode(w)
        } else {
            VarIntWrapper(self.count).encode(w)?;
            VarIntWrapper(self.item_id).encode(w)?;
            self.components.encode(w)
        }
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let count = VarIntWrapper::decode(r)?.0;
        if count <= 0 {
            return Ok(Slot {
                count: 0,
                item_id: 0,
                components: ComponentChanges::default(),
            });
        }

        let item_id = VarIntWrapper::decode(r)?.0;
        let components = ComponentChanges::decode(r)?;

        Ok(Slot {
            count,
            item_id,
            components,
        })
    }
}
