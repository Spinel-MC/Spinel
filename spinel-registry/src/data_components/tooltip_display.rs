use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, string_list_field_or_empty,
};
use crate::data_components::{DataComponentDescriptor, DataComponentValue};
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TooltipDisplay {
    hide_tooltip: bool,
    hidden_component_ids: BTreeSet<i32>,
}

impl TooltipDisplay {
    #[must_use]
    pub fn new(hide_tooltip: bool, hidden_component_ids: BTreeSet<i32>) -> Self {
        Self {
            hide_tooltip,
            hidden_component_ids,
        }
    }

    #[must_use]
    pub const fn hide_tooltip(&self) -> bool {
        self.hide_tooltip
    }

    #[must_use]
    pub fn hidden_component_ids(&self) -> &BTreeSet<i32> {
        &self.hidden_component_ids
    }

    #[must_use]
    pub fn with_hide_tooltip(&self, hide_tooltip: bool) -> Self {
        Self {
            hide_tooltip,
            hidden_component_ids: self.hidden_component_ids.clone(),
        }
    }

    #[must_use]
    pub fn with_hidden_component(&self, component: DataComponentDescriptor) -> Self {
        let mut hidden_component_ids = self.hidden_component_ids.clone();
        hidden_component_ids.insert(component.id());
        Self {
            hide_tooltip: self.hide_tooltip,
            hidden_component_ids,
        }
    }

    #[must_use]
    pub fn with(&self, component: DataComponentDescriptor) -> Self {
        self.with_hidden_component(component)
    }

    #[must_use]
    pub fn without_hidden_component(&self, component: DataComponentDescriptor) -> Self {
        let mut hidden_component_ids = self.hidden_component_ids.clone();
        hidden_component_ids.remove(&component.id());
        Self {
            hide_tooltip: self.hide_tooltip,
            hidden_component_ids,
        }
    }

    #[must_use]
    pub fn without(&self, component: DataComponentDescriptor) -> Self {
        self.without_hidden_component(component)
    }
}

impl DataComponentValue for TooltipDisplay {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.hide_tooltip {
            compound.insert("hide_tooltip".to_string(), Nbt::Byte(1));
        }
        if !self.hidden_component_ids.is_empty() {
            compound.insert(
                "hidden_components".to_string(),
                Nbt::List(
                    self.hidden_component_ids
                        .iter()
                        .filter_map(|component_id| DataComponentDescriptor::from_id(*component_id))
                        .map(|component| Nbt::String(component.key().to_string()))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let hidden_component_ids = string_list_field_or_empty(compound, "hidden_components")?
            .into_iter()
            .map(|component_key| {
                DataComponentDescriptor::from_key(&component_key).map(|component| component.id())
            })
            .collect::<Option<BTreeSet<_>>>()?;
        Some(Self {
            hide_tooltip: bool_field_or(compound, "hide_tooltip", false)?,
            hidden_component_ids,
        })
    }
}
