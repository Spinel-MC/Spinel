use crate::entity::Player;
use crate::inventory::slot_conversion::convert_minestom_slot_to_window_slot;
use crate::inventory::{Inventory, PlayerInventoryPacketSlot};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::container_set_content::ContainerSetContentPacket;
use spinel_core::network::clientbound::play::container_set_slot::ContainerSetSlotPacket;
use spinel_core::network::clientbound::play::open_screen::OpenScreenPacket;
use spinel_core::network::clientbound::play::set_cursor_item::SetCursorItemPacket;
use spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket;
use spinel_network::types::{Array, Slot};
use spinel_network::wrappers::NbtTextComponent;
use std::io;

impl Player {
    pub(crate) fn sync_open_inventory(&self, client: &mut Client) -> io::Result<()> {
        let Some(inventory) = self.get_opened_inventory() else {
            return Ok(());
        };
        self.send_open_inventory_packet(client, inventory)?;
        self.send_inventory_contents(client, inventory)
    }

    pub(crate) fn sync_open_inventory_contents(&self, client: &mut Client) -> io::Result<()> {
        let Some(inventory) = self.get_opened_inventory() else {
            return Ok(());
        };
        self.send_inventory_contents(client, inventory)
    }

    fn send_open_inventory_packet(
        &self,
        client: &mut Client,
        inventory: &Inventory,
    ) -> io::Result<()> {
        OpenScreenPacket {
            container_id: inventory.id(),
            container_type: inventory.inventory_type().window_type(),
            title: NbtTextComponent(inventory.title()),
        }
        .dispatch(client)
    }

    fn send_inventory_contents(
        &self,
        client: &mut Client,
        inventory: &Inventory,
    ) -> io::Result<()> {
        ContainerSetContentPacket {
            container_id: inventory.id(),
            state_id: 0,
            items: Array(
                inventory
                    .item_stacks()
                    .iter()
                    .map(Slot::from_item_stack)
                    .collect(),
            ),
            carried_item: Slot::from_item_stack(self.get_inventory_ref().cursor_item()),
        }
        .dispatch(client)
    }

    pub(crate) fn sync_inventory(&self, client: &mut Client) -> io::Result<()> {
        for (slot, item_stack) in self.get_inventory_ref().item_stacks().iter().enumerate() {
            match crate::inventory::PlayerInventory::packet_slot(slot as i32) {
                PlayerInventoryPacketSlot::PlayerInventory(packet_slot) => {
                    SetPlayerInventoryPacket {
                        slot: packet_slot,
                        item: Slot::from_item_stack(item_stack),
                    }
                    .dispatch(client)?;
                }
                PlayerInventoryPacketSlot::Window(packet_slot) => {
                    ContainerSetSlotPacket {
                        container_id: 0,
                        state_id: 0,
                        slot: packet_slot as i16,
                        item: Slot::from_item_stack(item_stack),
                    }
                    .dispatch(client)?;
                }
            }
        }
        SetCursorItemPacket {
            item: Slot::from_item_stack(self.get_inventory_ref().cursor_item()),
        }
        .dispatch(client)
    }

    pub(crate) fn sync_player_inventory_window_contents(
        &self,
        client: &mut Client,
    ) -> io::Result<()> {
        ContainerSetContentPacket {
            container_id: 0.into(),
            state_id: 0.into(),
            items: Array(self.player_inventory_window_slots()),
            carried_item: Slot::from_item_stack(self.get_inventory_ref().cursor_item()),
        }
        .dispatch(client)
    }

    fn player_inventory_window_slots(&self) -> Vec<Slot> {
        let mut window_slots = vec![
            Slot::from_item_stack(&spinel_registry::ItemStack::air());
            self.get_inventory_ref().get_size()
        ];
        for (slot, item_stack) in self.get_inventory_ref().item_stacks().iter().enumerate() {
            let window_slot = convert_minestom_slot_to_window_slot(slot as i32) as usize;
            window_slots[window_slot] = Slot::from_item_stack(item_stack);
        }
        window_slots
    }

    pub(crate) fn sync_slot(&self, slot: i32, client: &mut Client) -> io::Result<()> {
        let Some(item_stack) = self.get_item_at(slot) else {
            return Ok(());
        };
        if self.get_slot_is_in_open_inventory(slot) {
            let Some(inventory) = self.get_opened_inventory() else {
                return Ok(());
            };
            return ContainerSetSlotPacket {
                container_id: inventory.id(),
                state_id: 0,
                slot: self.get_window_slot(slot) as i16,
                item: Slot::from_item_stack(&item_stack),
            }
            .dispatch(client);
        }
        match crate::inventory::PlayerInventory::packet_slot(self.get_window_slot(slot)) {
            PlayerInventoryPacketSlot::PlayerInventory(packet_slot) => SetPlayerInventoryPacket {
                slot: packet_slot,
                item: Slot::from_item_stack(&item_stack),
            }
            .dispatch(client),
            PlayerInventoryPacketSlot::Window(packet_slot) => ContainerSetSlotPacket {
                container_id: 0,
                state_id: 0,
                slot: packet_slot as i16,
                item: Slot::from_item_stack(&item_stack),
            }
            .dispatch(client),
        }
    }

    pub(crate) fn sync_player_inventory_slot(
        &self,
        slot: i32,
        client: &mut Client,
    ) -> io::Result<()> {
        let Some(item_stack) = self.get_inventory_ref().get_item_stack(slot as usize) else {
            return Ok(());
        };
        match crate::inventory::PlayerInventory::packet_slot(slot) {
            PlayerInventoryPacketSlot::PlayerInventory(packet_slot) => SetPlayerInventoryPacket {
                slot: packet_slot,
                item: Slot::from_item_stack(item_stack),
            }
            .dispatch(client),
            PlayerInventoryPacketSlot::Window(packet_slot) => ContainerSetSlotPacket {
                container_id: 0,
                state_id: 0,
                slot: packet_slot as i16,
                item: Slot::from_item_stack(item_stack),
            }
            .dispatch(client),
        }
    }

    pub(crate) fn sync_cursor(&self, client: &mut Client) -> io::Result<()> {
        SetCursorItemPacket {
            item: Slot::from_item_stack(self.get_inventory_ref().cursor_item()),
        }
        .dispatch(client)
    }
}
