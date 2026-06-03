use crate::entity::{EquipmentSlot, Player};
use crate::inventory::ClickType;
use crate::inventory::slot_conversion::{CRAFT_RESULT, CRAFT_SLOT_1, CRAFT_SLOT_4, OFFHAND_SLOT};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::data_components::vanilla_components::EQUIPPABLE;
use spinel_registry::{ITEM_REGISTRY, Identifier, ItemStack, Material, Registries};
use std::collections::HashSet;
use std::sync::LazyLock;

impl Player {
    pub(super) fn apply_shift_click(
        &mut self,
        slot: i32,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(clicked) = self.item_at(slot) else {
            return false;
        };
        if clicked.is_air() {
            return true;
        }
        if self.shift_click_equips_item(slot, &clicked, player, server, client) {
            self.set_item_at_with_change_event(slot, ItemStack::air(), player, server, client);
            let cursor_item = self.inventory_ref().cursor_item().clone();
            let click_was_dispatched = self.dispatch_inventory_click(
                slot,
                ClickType::ShiftClick,
                &ItemStack::air(),
                &cursor_item,
                player,
                server,
                client,
            );
            return click_was_dispatched && self.resync_inventory_after_bulk_click(client);
        }
        let remaining_item = self.shift_click_remaining_item(slot, clicked, player, server, client);
        self.set_item_at_with_change_event(slot, remaining_item.clone(), player, server, client);
        let cursor_item = self.inventory_ref().cursor_item().clone();
        let click_was_dispatched = self.dispatch_inventory_click(
            slot,
            ClickType::ShiftClick,
            &remaining_item,
            &cursor_item,
            player,
            server,
            client,
        );
        click_was_dispatched && self.resync_inventory_after_bulk_click(client)
    }

    fn shift_click_equips_item(
        &mut self,
        slot: i32,
        clicked: &ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if self.opened_inventory().is_some() || self.is_crafting_slot(slot) {
            return false;
        }
        let Some(equipment_slot) = equipment_slot_for_item(clicked) else {
            return false;
        };
        if !self
            .inventory_ref()
            .equipment(equipment_slot, self.held_slot())
            .is_air()
        {
            return false;
        }
        let target_slot = match equipment_slot {
            EquipmentSlot::OffHand => OFFHAND_SLOT,
            _ => equipment_slot.armor_slot(),
        };
        if !self.set_item_at_with_change_event(target_slot, clicked.clone(), player, server, client)
        {
            return false;
        }
        let cursor_item = self.inventory_ref().cursor_item().clone();
        self.dispatch_inventory_click(
            target_slot,
            ClickType::ShiftClick,
            clicked,
            &cursor_item,
            player,
            server,
            client,
        )
    }

    fn shift_click_remaining_item(
        &mut self,
        slot: i32,
        clicked: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if self.opened_inventory().is_some() {
            return self.shift_click_with_open_inventory(slot, clicked, player, server, client);
        }
        self.shift_click_player_inventory(slot, clicked, player, server, client)
    }

    fn shift_click_with_open_inventory(
        &mut self,
        slot: i32,
        clicked: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        let open_inventory_size = self.open_inventory_size();
        if slot < open_inventory_size {
            let hotbar = ((open_inventory_size)..=(open_inventory_size + 8)).rev();
            let inventory = ((open_inventory_size + 9)..(open_inventory_size + 36)).rev();
            return self.move_item_to_slots(
                clicked,
                hotbar.chain(inventory),
                player,
                server,
                client,
            );
        }
        self.move_item_to_slots(clicked, 0..open_inventory_size, player, server, client)
    }

    fn shift_click_player_inventory(
        &mut self,
        slot: i32,
        clicked: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if self.is_equipment_slot(slot) || self.is_crafting_grid_slot(slot) {
            return self.move_item_to_slots(clicked, (9..36).chain(0..9), player, server, client);
        }
        if slot == CRAFT_RESULT {
            return self.move_item_to_slots(
                clicked,
                (0..=8).rev().chain((9..36).rev()),
                player,
                server,
                client,
            );
        }
        if slot < 9 {
            return self.move_item_to_slots(clicked, 9..36, player, server, client);
        }
        self.move_item_to_slots(clicked, 0..9, player, server, client)
    }

    fn is_crafting_slot(&self, slot: i32) -> bool {
        (CRAFT_RESULT..=CRAFT_SLOT_4).contains(&slot)
    }

    fn is_crafting_grid_slot(&self, slot: i32) -> bool {
        (CRAFT_SLOT_1..=CRAFT_SLOT_4).contains(&slot)
    }

    fn is_equipment_slot(&self, slot: i32) -> bool {
        (41..=OFFHAND_SLOT).contains(&slot)
    }
}

fn equipment_slot_for_item(item_stack: &ItemStack) -> Option<EquipmentSlot> {
    if let Some(equippable) = item_stack.get(EQUIPPABLE) {
        return EquipmentSlot::from_equippable_slot(equippable.slot());
    }
    let material_id = item_stack.material().id();
    if HEAD_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Helmet);
    }
    if CHEST_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Chestplate);
    }
    if LEG_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Leggings);
    }
    if FOOT_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Boots);
    }
    if OFFHAND_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::OffHand);
    }
    None
}

static HEAD_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> = LazyLock::new(|| {
    item_tag_entries("head_armor")
        .into_iter()
        .chain(item_ids([
            Material::CARVED_PUMPKIN,
            Material::CREEPER_HEAD,
            Material::DRAGON_HEAD,
            Material::PIGLIN_HEAD,
            Material::PLAYER_HEAD,
            Material::SKELETON_SKULL,
            Material::WITHER_SKELETON_SKULL,
            Material::ZOMBIE_HEAD,
        ]))
        .collect()
});

static CHEST_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> = LazyLock::new(|| {
    item_tag_entries("chest_armor")
        .into_iter()
        .chain(item_ids([Material::ELYTRA]))
        .collect()
});

static LEG_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> =
    LazyLock::new(|| item_tag_entries("leg_armor").into_iter().collect());

static FOOT_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> =
    LazyLock::new(|| item_tag_entries("foot_armor").into_iter().collect());

static OFFHAND_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> =
    LazyLock::new(|| item_ids([Material::SHIELD, Material::TOTEM_OF_UNDYING]).collect());

fn item_tag_entries(tag_path: &'static str) -> Vec<i32> {
    Registries::new_vanilla()
        .dynamic_tag_entries()
        .ok()
        .into_iter()
        .flatten()
        .find(|registry_tags| registry_tags.registry_name == ITEM_REGISTRY)
        .and_then(|registry_tags| {
            registry_tags
                .tags
                .into_iter()
                .find(|registry_tag| registry_tag.tag_name == Identifier::vanilla_static(tag_path))
        })
        .map(|registry_tag| registry_tag.entries)
        .unwrap_or_default()
}

fn item_ids<const N: usize>(materials: [Material; N]) -> impl Iterator<Item = i32> {
    materials.into_iter().map(|material| material.id())
}

#[cfg(test)]
mod tests {
    use super::Player;
    use crate::inventory::slot_conversion::OFFHAND_SLOT;
    use crate::inventory::{Inventory, InventoryType};
    use crate::network::client::instance::Client;
    use crate::server::MinecraftServer;
    use spinel_core::network::serverbound::play::container_click::ContainerClickPacket;
    use spinel_network::types::{Array, ItemStackHash};
    use spinel_registry::{ItemStack, Material};
    use spinel_utils::component::Component;
    use std::net::TcpListener;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use uuid::Uuid;

    #[test]
    fn player_inventory_shift_click_moves_hotbar_to_inventory() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND));
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
        assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
        assert_eq!(
            player.inventory_ref().item_stack(9).unwrap().material(),
            &Material::DIAMOND
        );
    }

    #[test]
    fn player_inventory_shift_click_moves_inventory_to_hotbar() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(9, ItemStack::of(Material::DIAMOND));
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(9, player_ptr, &mut server, &mut client));
        assert!(player.inventory_ref().item_stack(9).unwrap().is_air());
        assert_eq!(
            player.inventory_ref().item_stack(0).unwrap().material(),
            &Material::DIAMOND
        );
    }

    #[test]
    fn player_inventory_shift_click_moves_offhand_to_inventory_before_hotbar() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(OFFHAND_SLOT as usize, ItemStack::of(Material::DIAMOND));
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(OFFHAND_SLOT, player_ptr, &mut server, &mut client));
        assert!(
            player
                .inventory_ref()
                .item_stack(OFFHAND_SLOT as usize)
                .unwrap()
                .is_air()
        );
        assert_eq!(
            player.inventory_ref().item_stack(9).unwrap().material(),
            &Material::DIAMOND
        );
    }

    #[test]
    fn open_inventory_shift_click_uses_minestom_transfer_order() {
        let mut player = test_player();
        let mut inventory =
            Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
        inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND));
        player.open_inventory(inventory);
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
        assert!(
            player
                .opened_inventory()
                .unwrap()
                .item_stack(0)
                .unwrap()
                .is_air()
        );
        assert_eq!(
            player.inventory_ref().item_stack(8).unwrap().material(),
            &Material::DIAMOND
        );
    }

    #[test]
    fn player_inventory_shift_click_equips_registry_backed_head_items() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(0, ItemStack::of(Material::CARVED_PUMPKIN));
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
        assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
        assert_eq!(
            player
                .inventory_ref()
                .equipment(crate::entity::EquipmentSlot::Helmet, player.held_slot())
                .material(),
            &Material::CARVED_PUMPKIN
        );
    }

    #[test]
    fn player_inventory_shift_click_equips_extracted_component_helmet() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND_HELMET));
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
        assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
        assert_eq!(
            player
                .inventory_ref()
                .equipment(crate::entity::EquipmentSlot::Helmet, player.held_slot())
                .material(),
            &Material::DIAMOND_HELMET
        );
    }

    #[test]
    fn window_zero_hotbar_shift_click_equips_extracted_component_helmet() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND_HELMET));
        let mut server = MinecraftServer::new();
        let mut client = test_client();

        assert!(player.handle_container_click(
            &container_click_packet(36, 1, 0, 0),
            &mut server,
            &mut client
        ));
        assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
        assert_eq!(
            player
                .inventory_ref()
                .equipment(crate::entity::EquipmentSlot::Helmet, player.held_slot())
                .material(),
            &Material::DIAMOND_HELMET
        );
    }

    #[test]
    fn player_inventory_shift_click_equips_registry_backed_chest_items() {
        let mut player = test_player();
        player
            .inventory()
            .set_item_stack(0, ItemStack::of(Material::ELYTRA));
        let mut server = MinecraftServer::new();
        let mut client = test_client();
        let player_ptr = &mut player as *mut Player;

        assert!(player.apply_shift_click(0, player_ptr, &mut server, &mut client));
        assert!(player.inventory_ref().item_stack(0).unwrap().is_air());
        assert_eq!(
            player
                .inventory_ref()
                .equipment(crate::entity::EquipmentSlot::Chestplate, player.held_slot())
                .material(),
            &Material::ELYTRA
        );
    }

    fn test_player() -> Player {
        Player::new(
            Uuid::nil(),
            "Player".to_string(),
            0,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        )
    }

    fn test_client() -> Client {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
        let addr = listener.local_addr().unwrap();
        let stream = std::net::TcpStream::connect(addr).unwrap();
        let _ = listener.accept().unwrap();
        Client::new(stream, addr)
    }

    fn container_click_packet(
        slot: i16,
        click_type: i32,
        button: i8,
        container_id: i32,
    ) -> ContainerClickPacket {
        ContainerClickPacket {
            container_id,
            state_id: 0,
            slot,
            button,
            click_type,
            changed_slots: Array(Vec::new()),
            carried_item: ItemStackHash::from_item_stack(&ItemStack::air()),
        }
    }
}
