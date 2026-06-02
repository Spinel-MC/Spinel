use crate::entity::EquipmentSlot;
use spinel_core::network::clientbound::play::update_attributes::{
    EntityAttribute, EntityAttributeModifier, UpdateAttributesPacket,
};
use spinel_registry::data_components::vanilla_components::ATTRIBUTE_MODIFIERS;
use spinel_registry::{Attribute, AttributeOperation, Identifier, ItemStack};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LivingAttributes {
    instances: BTreeMap<i32, EntityAttributeState>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityAttributeState {
    attribute: Attribute,
    base_value: f64,
    modifiers: BTreeMap<String, EntityAttributeModifier>,
}

impl LivingAttributes {
    pub fn attribute(&mut self, attribute: Attribute) -> &mut EntityAttributeState {
        self.instances
            .entry(attribute.protocol_id())
            .or_insert_with(|| EntityAttributeState::new(attribute))
    }

    pub fn attributes(&self) -> Vec<&EntityAttributeState> {
        self.instances.values().collect()
    }

    pub fn attribute_value(&self, attribute: Attribute) -> f64 {
        self.instances
            .get(&attribute.protocol_id())
            .map(EntityAttributeState::value)
            .unwrap_or_else(|| attribute.default_value())
    }

    pub fn update_equipment_attributes(
        &mut self,
        previous_item_stack: &ItemStack,
        current_item_stack: &ItemStack,
        equipment_slot: EquipmentSlot,
    ) {
        self.remove_equipment_attributes(previous_item_stack, equipment_slot);
        self.add_equipment_attributes(current_item_stack, equipment_slot);
    }

    pub fn packet(&self, entity_id: i32) -> UpdateAttributesPacket {
        UpdateAttributesPacket {
            entity_id,
            attributes: self
                .instances
                .values()
                .filter(|attribute| attribute.attribute().is_syncable())
                .map(EntityAttributeState::packet_attribute)
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    fn remove_equipment_attributes(
        &mut self,
        item_stack: &ItemStack,
        equipment_slot: EquipmentSlot,
    ) {
        let Some(attribute_list) = item_stack.get(ATTRIBUTE_MODIFIERS) else {
            return;
        };
        attribute_list
            .modifiers()
            .iter()
            .filter(|modifier| {
                modifier
                    .slot()
                    .contains_slot_name(equipment_slot.nbt_name())
            })
            .filter_map(|modifier| {
                Attribute::from_identifier(modifier.attribute_type())
                    .map(|attribute| (attribute, modifier.id()))
            })
            .for_each(|(attribute, modifier_id)| {
                self.attribute(attribute).remove_modifier(modifier_id);
            });
    }

    fn add_equipment_attributes(&mut self, item_stack: &ItemStack, equipment_slot: EquipmentSlot) {
        let Some(attribute_list) = item_stack.get(ATTRIBUTE_MODIFIERS) else {
            return;
        };
        attribute_list
            .modifiers()
            .iter()
            .filter(|modifier| {
                modifier
                    .slot()
                    .contains_slot_name(equipment_slot.nbt_name())
            })
            .filter_map(|modifier| {
                Attribute::from_identifier(modifier.attribute_type()).map(|attribute| {
                    (
                        attribute,
                        EntityAttributeModifier {
                            modifier_id: modifier.id().clone(),
                            amount: modifier.amount(),
                            operation: modifier.operation().protocol_id(),
                        },
                    )
                })
            })
            .for_each(|(attribute, modifier)| {
                self.attribute(attribute).add_modifier(modifier);
            });
    }
}

impl EntityAttributeState {
    pub fn new(attribute: Attribute) -> Self {
        Self {
            attribute,
            base_value: attribute.default_value(),
            modifiers: BTreeMap::new(),
        }
    }

    pub const fn attribute(&self) -> Attribute {
        self.attribute
    }

    pub const fn attribute_id(&self) -> i32 {
        self.attribute.protocol_id()
    }

    pub const fn base_value(&self) -> f64 {
        self.base_value
    }

    pub fn set_base_value(&mut self, base_value: f64) {
        self.base_value = base_value;
    }

    pub fn add_modifier(&mut self, modifier: EntityAttributeModifier) {
        self.modifiers
            .insert(modifier.modifier_id.to_string(), modifier);
    }

    pub fn remove_modifier(&mut self, modifier_id: &Identifier) -> Option<EntityAttributeModifier> {
        self.modifiers.remove(&modifier_id.to_string())
    }

    pub fn modifiers(&self) -> Vec<&EntityAttributeModifier> {
        self.modifiers.values().collect()
    }

    pub fn value(&self) -> f64 {
        let add_value_total = self
            .modifiers
            .values()
            .filter(|modifier| modifier.operation == AttributeOperation::AddValue.protocol_id())
            .map(|modifier| modifier.amount)
            .sum::<f64>();
        let additive_base = self.base_value + add_value_total;
        let add_multiplied_base_total = self
            .modifiers
            .values()
            .filter(|modifier| {
                modifier.operation == AttributeOperation::AddMultipliedBase.protocol_id()
            })
            .map(|modifier| additive_base * modifier.amount)
            .sum::<f64>();
        self.modifiers
            .values()
            .filter(|modifier| {
                modifier.operation == AttributeOperation::AddMultipliedTotal.protocol_id()
            })
            .fold(
                additive_base + add_multiplied_base_total,
                |attribute_value, modifier| attribute_value * (1.0 + modifier.amount),
            )
            .clamp(self.attribute.min_value(), self.attribute.max_value())
    }

    fn packet_attribute(&self) -> EntityAttribute {
        EntityAttribute {
            attribute_id: self.attribute.protocol_id(),
            base_value: self.base_value,
            modifiers: self.modifiers.values().cloned().collect(),
        }
    }
}
