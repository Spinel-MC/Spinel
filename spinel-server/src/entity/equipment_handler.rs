use crate::entity::{EntityId, EquipmentSlot, Error};
use crate::world::World;
use spinel_registry::ItemStack;

pub trait EquipmentHandler {
    fn get_entity_id(&self) -> EntityId;

    fn get_equipment(&self, equipment_slot: EquipmentSlot) -> ItemStack;

    fn set_equipment(
        &mut self,
        world: &mut World,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Result<(), Error> {
        let was_applied =
            world.set_entity_equipment(self.get_entity_id(), equipment_slot, item_stack)?;
        if !was_applied {
            return Err(Error::EquipmentMutationRejected);
        }
        Ok(())
    }

    fn get_item_in_main_hand(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::MainHand)
    }

    fn set_item_in_main_hand(
        &mut self,
        world: &mut World,
        item_stack: ItemStack,
    ) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::MainHand, item_stack)
    }

    fn get_item_in_off_hand(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::OffHand)
    }

    fn set_item_in_off_hand(
        &mut self,
        world: &mut World,
        item_stack: ItemStack,
    ) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::OffHand, item_stack)
    }

    fn get_item_in_hand(&self, hand: crate::entity::PlayerHand) -> ItemStack {
        match hand {
            crate::entity::PlayerHand::Main => self.get_item_in_main_hand(),
            crate::entity::PlayerHand::Off => self.get_item_in_off_hand(),
        }
    }

    fn set_item_in_hand(
        &mut self,
        world: &mut World,
        hand: crate::entity::PlayerHand,
        item_stack: ItemStack,
    ) -> Result<(), Error> {
        match hand {
            crate::entity::PlayerHand::Main => self.set_item_in_main_hand(world, item_stack),
            crate::entity::PlayerHand::Off => self.set_item_in_off_hand(world, item_stack),
        }
    }

    fn get_helmet(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::Helmet)
    }

    fn set_helmet(&mut self, world: &mut World, item_stack: ItemStack) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::Helmet, item_stack)
    }

    fn get_chestplate(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::Chestplate)
    }

    fn set_chestplate(&mut self, world: &mut World, item_stack: ItemStack) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::Chestplate, item_stack)
    }

    fn get_leggings(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::Leggings)
    }

    fn set_leggings(&mut self, world: &mut World, item_stack: ItemStack) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::Leggings, item_stack)
    }

    fn get_boots(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::Boots)
    }

    fn set_boots(&mut self, world: &mut World, item_stack: ItemStack) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::Boots, item_stack)
    }

    fn get_body_equipment(&self) -> ItemStack {
        self.get_equipment(EquipmentSlot::Body)
    }

    fn set_body_equipment(
        &mut self,
        world: &mut World,
        item_stack: ItemStack,
    ) -> Result<(), Error> {
        self.set_equipment(world, EquipmentSlot::Body, item_stack)
    }
    fn has_equipment(&self, equipment_slot: EquipmentSlot) -> bool {
        !self.get_equipment(equipment_slot).is_air()
    }
}
